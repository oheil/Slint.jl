
// set RUST_LOG=debug

use log::*;
use env_logger::Env;


use slint::{Model, ModelRc, ModelTracker, ModelNotify};
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::ffi::{CStr, CString, c_char};
use std::rc::Weak;

//use slint_interpreter::{Weak, Value, ValueType, ComponentCompiler, ComponentInstance, ComponentHandle, SharedString};
use slint_interpreter::Compiler;
use slint_interpreter::ComponentHandle;
use slint_interpreter::Value;


pub fn main() {
    let env = Env::default()
        .filter_or("RUST_LOG", "info");
    env_logger::init_from_env(env);

    let compiler = Compiler::default();

    let code = r#"
    // Copyright © SixtyFPS GmbH <info@slint.dev>
    // SPDX-License-Identifier: MIT
    
    import { Button, LineEdit, ScrollView, ListView, GridBox} from "std-widgets.slint";
    
    struct SlintValue  { value_s: string, value_i: int }
    
    export component MainWindow inherits Window {
        width: 500px;
        height: 500px;
        
        in property <[[SlintValue]]> cells;
    
        private property <length> cell-height: 32px;
        private property <length> cell-width: 100px;
        private property <{r: int, c: int}> active-cell: { r: -1, c: -1 };
        
        callback add_row();

        GridBox {
            Button {
                row: 0;
                width: 100px;
                text: "Add Row";
                clicked => { root.add_row() }
            }
            ScrollView {
                row: 1;
                width: 100%;
                height: 100%;
                viewport-width: 20px + 26 * root.cell-width;
                viewport-height: 100 * root.cell-height;
        
                for letter[idx] in ["A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M",
                                    "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z" ] : Rectangle {
                    y:0;
                    x: 20px + idx * root.cell-width;
                    height: root.cell-height;
                    width: root.cell-width;
                    Text { x:0;y:0; text: letter; }
                }
                for row[row-idx] in root.cells : Rectangle {
                    y: root.cell-height + row-idx * root.cell-height;
                    height: root.cell-height;
        
                    Text { x:0;y:0; text: row_idx; }
        
                    for cell[col-idx] in row: Rectangle {
                        property <bool> is-active: root.active-cell.r == row-idx && root.active-cell.c == col-idx;
        
                        y:0;
                        height: root.cell-height;
                        width: root.cell-width;
                        border-color: gray;
                        border-width: 1px;
                        x: 20px + col-idx * root.cell-width;
        
                        Text {
                            visible: !is-active;
                            text: " " + cell.value_s;
                            vertical-alignment: center;
                            width: 100%;
                            height: 100%;
                        }
        
                        TouchArea {
                            clicked => {
                                l.text = cell.value_s;
                                root.active-cell = {r: row-idx, c: col-idx};
                                l.focus();
                            }
                        }
        
                        l := LineEdit {
                            edited => {
                                cell = { value_s: self.text };
                            }
                            accepted => {
                                root.active-cell = { r: -1, c: -1};
                            }
        
                            visible: is-active;
                            width: 100%;
                            height: 100%;
                        }
                    }
                }
            }
        } 
    }
    
//    export component Cell inherits MainWindow {
//        // initialize the cells with demy value to be viewed in the preview
//        in-out property <[SlintValue]> _row1: [{}, {},];
//        in-out property <[SlintValue]> _row2: [{}, {},];
//        cells: [
//            root._row1, root._row2,
//        ];
//    }

    "#;

    let result = spin_on::spin_on(
        compiler.build_from_source(code.into(), Default::default()));

    result.print_diagnostics();

    let definition = result.component("MainWindow");
    let instance = definition.unwrap().create().unwrap();

    let cells_model = CellsModel::new(2,2);
    
    let r = instance.set_property("cells", Value::Model(cells_model.clone().into()) );
    match r {
        Ok(_) => (),
        Err(error) => warn!("main:setting model for property <{}> failed: {:?}", "cells", error),
    };

    let instance_weak = instance.as_weak();
    let _ = instance.set_callback("add_row", move |_| {
        let cells = instance_weak.unwrap().get_property("cells").unwrap();

        debug!("on_add_row {:#?}",cells);
        debug!("on_add_row {:#?}",cells_model.rows.borrow()[0].row_data(0) );
        
        //Unfortunately you can't get a mutable reference to `the_model`. 
        //You'll have to stay with an immutable `&` reference, change `add_row` to take `&self`, 
        //and use interior mutability for any mutations needed in `CellsModel`. 
        //For example if `self.rows` as a `slint::VecModel`, you could call `push` with `&self`, `&mut self` is not required.
        //https://chat.slint.dev/public/pl/85hgpz9rf3fwxdjt5exktu16ic

        //let the_model = &mut cells_model.as_any().downcast_ref::<Rc<CellsModel>>().expect("We know we set it");
        
        let _ = cells_model.add_row();
        
        cells_model.notify.row_changed( cells_model.rows.borrow().len() );

        instance_weak.unwrap().window().request_redraw();

        return Value::Void;
    });

    instance.run().unwrap();

    /*
    if compiler.diagnostics().is_empty() {
        if let Some(definition) = definition {
            let instance = definition.create().unwrap();
            let r = instance.get_property("cells");
            match r {
                Ok(_) => {
					info!("OK");
					()
				},
                Err(error) => info!("get_property: {:?}", error),
            };
        }
    }
    */

}

// debug output
fn print_type_of<T>(_: &T) {
    debug!("print_type_of:type: {}", std::any::type_name::<T>())
}

//
// JRvalue is used to receive return value from Julia callbacks
//   and as a return value to calls from Julia (e.g. r_get_cell_value) if helpfull
//
const JRMAGIC: i32 = 123456;
#[no_mangle]
pub unsafe extern "C" fn r_get_magic() -> i32 {
    debug!("r_get_magic");
    JRMAGIC
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct JRvalue {
    magic: i32,
    rtype: *const c_char,
    int_value: i32,
    string_value: *const c_char,
}
impl JRvalue {
    fn new_bool(b: bool) -> Self {
        debug!("JRvalue.new_bool");
        JRvalue {
            magic: JRMAGIC,
            rtype: CString::new("Bool").unwrap().into_raw(),
            int_value: b.into(),
            string_value: CString::new("").unwrap().into_raw()
        }
    }
    fn new_undefined() -> Self {
        debug!("JRvalue.new_undefined");
        JRvalue {
            magic: JRMAGIC,
            rtype: CString::new("Unknown").unwrap().into_raw(),
            int_value: 0,
            string_value: CString::new("").unwrap().into_raw()
        }
    }
    /*
    fn from_ref(rv_ref: &Self) -> Self {
        debug!("JRvalue.from_ref");
        JRvalue {
            magic: (*rv_ref).magic,
            rtype: (*rv_ref).rtype,
            int_value: (*rv_ref).int_value,
            string_value: (*rv_ref).string_value,
        }
    }
    */
}

//
// below the generic model for every slint 2-dimensional vector property
//   1-dimension vectors are handled like 2 dimensions with length 1 of one dimension
//   cell/element values are always strings
//
#[derive(Clone)]
struct SlintValue  { 
    value_s: String,
    value_i: i32,
}
impl Default for SlintValue {
    fn default() -> SlintValue {
        debug!("SlintValue");
        SlintValue{
            value_s: String::from(""),
            value_i: 0,
        }
    }
}

struct CellsModel {
    rows: RefCell<Vec<Rc<RowModel>>>,
    notify: ModelNotify,
}

impl Model for CellsModel {
    type Data = Value;  // Data is Value

    fn row_count(&self) -> usize {
        debug!("CellsModel.row_count");
        debug!("CellsModel.row_count: {}",self.rows.borrow().len());
        self.rows.borrow().len()
    }

    fn row_data(&self, row: usize) -> Option<Self::Data> {
        debug!("CellsModel.row_data");
        // maps the data to a Value
        self.rows.borrow().get(row).map(|x| Value::Model(ModelRc::new(x.clone())))
    }
    fn model_tracker(&self) -> &dyn ModelTracker {
        debug!("CellsModel.model_tracker");
        &()
    }
}
impl CellsModel {
    fn new(nrows: usize, ncols: usize) -> Rc<Self> {
        debug!("CellsModel.new");
        Rc::new_cyclic(|w| Self {
            rows: RefCell::new((0..nrows)
                .map(|row| {
                    Rc::new(RowModel {
                        row,
                        row_elements: vec![SlintValue::default(); ncols].into(),
                        base_model: w.clone(),
                        notify: Default::default(),
                    })
                })
                .collect()),
                notify: Default::default(),
        })
    }
    
    fn add_row(&self) -> Option<()> {
        let row_count = self.row_count() + 1;
        let col_count = self.col_count();
        
        let w = Weak::<CellsModel>::new();

        let row = Rc::new(RowModel {
            row: row_count,
            row_elements: vec![SlintValue::default(); col_count].into(),
            base_model: w.clone(),
            notify: Default::default(),
        });

        let mut rows_mut = self.rows.borrow_mut();
        rows_mut.push(row);
        
        let row_model = rows_mut.get(row_count)?;
        row_model.notify.row_changed(2);

        self.notify.row_added(row_count, col_count);

        //let rows = self.rows.borrow();
        //let r_model = rows.get(row_count)?;
        //r_model.notify.row_changed(0);

        Some(())
    }

    fn col_count(&self) -> usize {
        debug!("CellsModel.col_count");
        debug!("CellsModel.col_count: {}",self.rows.borrow().len());
        let mut r: usize = 0;
        if self.rows.borrow().len() > 0 {
            r = self.rows.borrow().get(0).unwrap().row_count()
        }
        r
    }

    //fn get_cell_value(&self, row: usize, col: usize) -> Option<String> {
    fn get_cell_value(&self, row: usize, col: usize) -> Option<JRvalue> {
        debug!("CellsModel.get_cell_value");
        debug!("CellsModel.get_cell_value: row={} col={}",row+1,col+1);
        if row >= self.row_count() {
            warn!("CellsModel.get_cell_value: row index <{}> not in range of existing row indices <1..{}>",row+1,self.row_count());
        }
        if col >= self.col_count() {
            warn!("CellsModel.get_cell_value: col index <{}> not in range of existing column indices <1..{}>",col+1,self.col_count());
        }
        //let v: String = self.rows.get(row)?.row_elements.borrow().get(col)?.value_s.clone();
        let mut rv = JRvalue::new_undefined();
        //rv.string_value = self.rows.get(row)?.row_elements.borrow().get(col)?.value_s.clone();
        rv.string_value = CString::new(self.rows.borrow().get(row)?.row_elements.borrow().get(col)?.value_s.clone()).unwrap().into_raw();
        rv.int_value = self.rows.borrow().get(row)?.row_elements.borrow().get(col)?.value_i;
        Some(rv)
    }

    //fn update_cell(&self, row: usize, col: usize, new_value: Option<SharedString>) -> Option<()> {
    fn update_cell(&self, row: usize, col: usize, new_value: Option<JRvalue>) -> Option<()> {
        debug!("CellsModel.update_cell");
        //debug!("CellsModel.update_cell: row={} col={} new_value={}",row+1,col+1,new_value.as_ref().unwrap());
        match new_value {
            Some(new_v) => {
                debug!("CellsModel.update_cell: row={} col={}",row+1,col+1);
                debug!("CellsModel.update_cell: new_v.int_value={}",new_v.int_value);
                debug!("CellsModel.update_cell: new_v.string_value={:p}",new_v.string_value);
                if row >= self.row_count() {
                    warn!("CellsModel.update_cell: row index <{}> not in range of existing row indices <1..{}>",row+1,self.row_count());
                }
                if col >= self.col_count() {
                    warn!("CellsModel.update_cell: col index <{}> not in range of existing column indices <1..{}>",col+1,self.col_count());
                }
                let rows = self.rows.borrow();
                let r_model = rows.get(row)?;
                let mut row_el = r_model.row_elements.borrow_mut();
                let data = row_el.get_mut(col)?;

                let mut rv = JRvalue::new_bool(true);
                unsafe {
                    let args = &[
                        Value::Number(((row+1) as i32).into()),
                        Value::Number(((col+1) as i32).into()),
                        //Value::String(new_value.as_ref().unwrap().clone()),
                        Value::String(CStr::from_ptr(new_v.string_value).to_string_lossy().into_owned().into()),
                        Value::String(data.value_s.clone().into())
                        ];
            
                    drop(row_el);
                }
        
                // debug JRvalue returned
                print_type_of(&rv);
                debug!("CellsModel.update_cell:return value magic is: {}", rv.magic);
                debug!("CellsModel.update_cell:return value type is: {:p}", rv.rtype);
                debug!("CellsModel.update_cell:return value int_value is: {}", rv.int_value);
                debug!("CellsModel.update_cell:return value string_value is: {:p}", rv.string_value);
                // debug end
        
                // valid JRvalue only if magic == 123456
                if rv.magic == JRMAGIC {
                    unsafe {
                        // get the type of the return value
                        let rv_cstr = CStr::from_ptr(rv.rtype);
                        let rv_type: String = rv_cstr.to_string_lossy().into_owned();
        
                        debug!("CellsModel.update_cell: rv_type={}", rv_type);
        
                        // create a Slint::Value from JRvalue as a valid Slint return value of a callback
                        if rv_type == "Bool" {
                            if rv.int_value == 0 { // false => do not change cell value
                                return Some(());
                            }
                        } else {
                            error!("CellsModel.update_cell:callback return value of type {} is not implemented",rv_type);
                        }
                    }
                } else {
                    error!("CellsModel.update_cell:callback must return a valid JRvalue, JRvalue.magic must equal {}",JRMAGIC);
                }
        
                let mut row_el = r_model.row_elements.borrow_mut();
                let data = row_el.get_mut(col)?;
        
                debug!("CellsModel.update_cell: data.value_s={}",data.value_s);
                debug!("CellsModel.update_cell: data.value_i={}",data.value_i);

                unsafe {
                    data.value_s = CStr::from_ptr(new_v.string_value).to_string_lossy().into_owned();
                }
        
                drop(row_el);
                r_model.notify.row_changed(col);
        
            },
            None => {
                debug!("update_cell:no new value");

            },
        }
        Some(())
    }
}
struct RowModel {
    row: usize,
    row_elements: RefCell<Vec<SlintValue>>,
    base_model: std::rc::Weak<CellsModel>,
    notify: ModelNotify,
}
impl slint::Model for RowModel {
    type Data = Value; // again, Data must be Value

    fn row_count(&self) -> usize {
        debug!("RowModel.row_count");
        debug!("RowModel.row_count: {}",self.row_elements.borrow().len());
        self.row_elements.borrow().len()
    }

    fn row_data(&self, row: usize) -> Option<Self::Data> {
        debug!("RowModel.row_data");
        debug!("RowModel.row_data: row={}",row+1);
        self.row_elements.borrow().get(row).map(|row_element| {
            debug!("RowModel.row_data: row_element.value_s={}",row_element.value_s);
            let mut stru = slint_interpreter::Struct::default();
            stru.set_field("value_s".into(), Value::String(row_element.value_s.clone().into()));
            stru.set_field("value_i".into(), Value::Number(row_element.value_i.into()));
            stru.into()
        })
    }

    fn model_tracker(&self) -> &dyn ModelTracker {
        debug!("RowModel.model_tracker");
        &self.notify
    }

    fn set_row_data(&self, row: usize, data: Value) {
        debug!("RowModel.set_row_data");
        debug!("RowModel.set_row_data: row={} data.value_type={:#?}",row+1,data.value_type());
        if let Some(cells) = self.base_model.upgrade() {
            let stru = slint_interpreter::Struct::try_from(data).unwrap();
            let val = stru.get_field("value_s".into()).unwrap().clone();
            let shstr = slint_interpreter::SharedString::try_from(val).unwrap();
            let mut rv = JRvalue::new_undefined();
            rv.string_value = CString::new(shstr.as_str()).unwrap().into_raw();
            //cells.update_cell(self.row, row, Some(shstr));
            cells.update_cell(self.row, row, Some(rv));
        }
    }
}

//
// API to models for arrays/matrices ends here
//

