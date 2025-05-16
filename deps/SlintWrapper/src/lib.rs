use std::sync::Mutex;
use once_cell::sync::Lazy;
use std::ffi::{CStr, CString, c_void, c_char};
use std::convert::From;

use log::*;
use env_logger::Env;

//use slint_interpreter::{Weak, Value, ValueType, ComponentCompiler, ComponentInstance, ComponentHandle, SharedString};
use slint_interpreter::{Weak, Value, ValueType, Compiler, ComponentInstance, ComponentHandle };
use slint::{Model, ModelRc, ModelTracker, ModelNotify, SharedString};
use slint::StandardListViewItem;
use slint::VecModel;

use std::cell::RefCell;
use std::rc::Rc;

mod slint_value;
pub use crate::slint_value::*;

// only hold a single instance at index 0
static INSTANCES: Lazy<Mutex<Vec<Weak<ComponentInstance>>>> = Lazy::new(|| {
    Mutex::new(Vec::new())
});

// debug output
fn print_type_of<T>(_: &T) {
    debug!("print_type_of:type: {}", std::any::type_name::<T>())
}

// do anything needed at startup
#[no_mangle]
pub extern "C" fn r_init() {
    debug!("r_init");
    let env = Env::default()
        .filter_or("RUST_LOG", "info");
    env_logger::init_from_env(env);
}

//
// compile a .slint file, create the single instance
// but do not run it yet, to be able to set callbacks
//
#[no_mangle]
pub unsafe extern "C" fn r_compile_from_file(slint_file: *const c_char, slint_comp: *const c_char) {
    debug!("r_compile_from_file");
    let cstr = CStr::from_ptr(slint_file);
    let filename: String = cstr.to_string_lossy().into_owned();

    let start_component = CStr::from_ptr(slint_comp).to_str().unwrap();
    
    //let mut compiler = ComponentCompiler::default();
    let compiler = Compiler::default();
    //compilers.push(&compiler);

    let result = spin_on::spin_on(
        compiler.build_from_path(filename)
    );
    //definitions.push(&definition);

    //let code = r#"
    //    ...
    //"#;
    //let definition = spin_on::spin_on(
    //    compiler.build_from_source(code.into(), Default::default()));
    let diagnostics : Vec<_> = result.diagnostics().collect();
    if diagnostics.is_empty() {
        debug!("r_compile_from_file: diagnostics is empty");
        if let Some(definition) = result.component(start_component) {
            let instance = definition.create().unwrap();
            if ! INSTANCES.lock().unwrap().is_empty() {
                INSTANCES.lock().unwrap().pop();
            }
            INSTANCES.lock().unwrap().push(instance.as_weak());

            // shows the window on the screen and maintains an extra strong reference.
            // if not called, the instance is dropped and lost.
            let _ = instance.show();


            /*
            let model = CellsModel::new(1,1,None);
            let row_count = model.row_count()+1;
            let some_row = model.rows.borrow()[0].clone();
            let mut values: Vec<StandardListViewItem> = Vec::new();
            let mut sv = StandardListViewItem::default();
            sv.text = SharedString::from("TEST1");
            values.push(sv.clone());
            sv.text = SharedString::from("TEST2");
            values.push(sv.clone());

            let values2: Vec<SlintValue> = values.vec_into();
            debug!("{}",values2[0].value_slvi.text);
            debug!("{}",values2[1].value_slvi.text);

            let values3: Vec<StandardListViewItem> = values2.vec_into();
            debug!("{}",values3[0].text);
            debug!("{}",values3[1].text);

            let _new_row = Rc::new(RowModel {
                row: row_count,
                row_elements: values3.vec_into().into(),
                base_model: some_row.base_model.clone(),
                notify: Default::default(),
                func: some_row.func,
            });

            model.push_row(_new_row);
            //model.remove_row(0);

            //model.push_row("names-list",[entry3])

            //let slvi_list = ModelRc::new(VecModel::from(vec![sv.clone(),sv.clone()]));
            
            //let _ = instance.set_property("names-list", slvi_list.into());
            
            */

            
            let _ = instance.set_callback("bridge2StandardListViewItem", move |args: &[Value]| -> Value {
                debug!("bridge2StandardListViewItem");
                let ss = SharedString::try_from(args[0].clone()).unwrap();
                let propertyid: String = ss.as_str().to_string();
                
                let source_model: Rc<CellsModel> = model_get(&propertyid);
                debug!("bridge2StandardListViewItem:source_model.row_count(): {}",source_model.row_count());
                print_type_of(&source_model);

                let ss2 = SharedString::try_from(args[1].clone()).unwrap();
                let propertyid2: String = ss2.as_str().to_string();

        
                //let ss: SharedString = SharedString::try_from("Hello").unwrap();
                //let v: StandardListViewItem = StandardListViewItem::from(ss);
                //let model2 = ModelRc::new(VecModel::from(vec![ss.clone(),ss.clone()]));

                //let sv: StandardListViewItem = StandardListViewItem::try_from(ss).unwrap();
                //let slvi_list: Vec<StandardListViewItem> = vec![sv.clone(),sv.clone()];

                let mut slvi_list: Vec<StandardListViewItem> = vec![];
                for (rowindex, row) in source_model.rows.borrow().iter().enumerate() {
                    if rowindex > 0 {
                        for (_cellindex, cell) in row.row_elements.borrow().iter().enumerate() {
                            let ss = SharedString::try_from(cell.value_s.clone()).unwrap();
                            let sv: StandardListViewItem = StandardListViewItem::try_from(ss).unwrap();
                            slvi_list.push(sv);
                        }
                    }
                }

                let new_model = ModelRc::new(VecModel::from(slvi_list));
                let instance2 = (&(INSTANCES.lock().unwrap())[0]).upgrade();

                //let slvi_list = ModelRc::new(VecModel::from(vec![sv.clone(),sv.clone()]));
                //let _ = instance2.unwrap().set_property("names-list", slvi_list.into());

                let r = instance2.unwrap().set_property(&propertyid2, new_model.into());
                match r {
                    Ok(_) => (),
                    Err(error) => warn!("bridge2StandardListViewItem:setting model for property <{}> failed: {:?}", propertyid2, error),
                };
    


                return Value::from(Value::Void);
            } );
            

            /*
            let _ = instance.set_callback("bridge2StandardListViewItem", move |args: &[Value]| -> Value {
                debug!("bridge2StandardListViewItem");

                //method1
                let mut rowmodel2: ModelRc<Value> = ModelRc::try_from(args[1].clone()).unwrap();
                debug!("bridge2StandardListViewItem:rowmodel2.row_count(): {}",rowmodel2.row_count());

                //method2
                if let slint_interpreter::Value::Model(rowmodel) = &args[0] {
                    debug!("bridge2StandardListViewItem:rowmodel.row_count(): {}",rowmodel.row_count());
                    print_type_of(rowmodel);

                    let row1 = rowmodel.row_data(1).unwrap();
                    print_type_of(&row1);
                    
                    if let slint_interpreter::Value::Model(cellmodel) = row1 {
                        debug!("bridge2StandardListViewItem:cellmodel.row_count(): {}",cellmodel.row_count());

                        let el1 = cellmodel.row_data(0).unwrap();
                        print_type_of(&el1);

                        let stru = slint_interpreter::Struct::try_from(el1).unwrap();
                        let val = stru.get_field("value_s".into()).unwrap().clone();
                        let shstr = slint_interpreter::SharedString::try_from(val).unwrap();

                        debug!("bridge2StandardListViewItem: el1 {:#?}",shstr);

                        let slsvi: StandardListViewItem = StandardListViewItem::try_from(shstr).unwrap();
                        
                        print_type_of(&rowmodel2);

                        let ss: SharedString = SharedString::try_from("Hello").unwrap();
                        let v: Value = Value::from(ss);

                        let new_rowmodel = Rc::new(VecModel::from(vec![v.clone(),v.clone()]));
                        let the_model_rc = ModelRc::from(new_rowmodel.clone());

                        rowmodel2 = the_model_rc;
                        print_type_of(&rowmodel2);

                        


                    }

                    //let some_elem = row1.get(0);
                    //debug!("bridge2StandardListViewItem: some_row {:#?}",some_elem.unwrap().value_s);
                }
               

                return Value::from(Value::Void);
            } );
            */

            /*
            let _ = instance.set_callback("bridge2StandardListViewItem", move |args: &[Value]| -> Value {
                debug!("bridge2StandardListViewItem");
                // debug list of arguments
                debug!("bridge2StandardListViewItem:slint calls bridge2StandardListViewItem with {} arguments", args.len());
                for arg in args {
                    print_type_of(arg);
                    let vt = arg.value_type();
                    debug!("bridge2StandardListViewItem:value type is: {:#?}", vt);
                }
                // debug end

                // create a void ptr to list of arguments which is send to Julias callback.
                let args_ptr: *const c_void = args as *const [Value] as *const c_void;
                // get number of arguments. This is send to Julia callback, too.
                let len: i32 = args.len().try_into().unwrap();

                // debug void ptr
                debug!("bridge2StandardListViewItem:void ptr adress is: {:p}", args_ptr);
                let args2: &[Value] = std::slice::from_raw_parts(args_ptr as *const Value, args.len());
                for arg in args2 {
                    print_type_of(arg);
                    let vt = arg.value_type();
                    debug!("bridge2StandardListViewItem:value type is: {:#?}", vt);
                }

                let sv = &args2[0];
                
                //let model: Rc<CellsModel> = sv.clone().try_into().unwrap();
                let propertyid: String = String::from("names-list-bridge");
                let model: Rc<CellsModel> = model_get(&propertyid);
                let some_row = model.rows.borrow()[1].clone();
                let binding = some_row.row_elements.borrow();
                let some_elem = binding.get(0);
                debug!("bridge2StandardListViewItem: some_row {:#?}",some_elem.unwrap().value_s);
                let some_elem2 = binding.get(1);
                debug!("bridge2StandardListViewItem: some_row {:#?}",some_elem2.unwrap().value_s);
                let some_row2 = model.rows.borrow()[2].clone();
                let binding2 = some_row2.row_elements.borrow();
                let some_elem3 = binding2.get(0);
                debug!("bridge2StandardListViewItem: some_row {:#?}",some_elem3.unwrap().value_s);


                return Value::from(Value::Void);
            } );
            */





            


        }
    } else {
        debug!("r_compile_from_file: diagnostics is not empty");
        //slint_interpreter::print_diagnostics(&compiler.diagnostics());
        result.print_diagnostics();
    }
}

//
// compile from string with slint code, create the single instance
// but do not run it yet, to be able to set callbacks
//
#[no_mangle]
pub unsafe extern "C" fn r_compile_from_string(slint_string: *const c_char, slint_comp: *const c_char) {
    debug!("r_compile_from_string");
    let cstr = CStr::from_ptr(slint_string);
    let slint_code: String = cstr.to_string_lossy().into_owned();

    let start_component = CStr::from_ptr(slint_comp).to_str().unwrap();

    //let mut compiler = ComponentCompiler::default();
    let compiler = Compiler::default();

    let result = spin_on::spin_on(compiler.build_from_source(slint_code.into(), Default::default()));

    let diagnostics : Vec<_> = result.diagnostics().collect();
    if diagnostics.is_empty() {
        if let Some(definition) = result.component(start_component) {
            let instance = definition.create().unwrap();
            if ! INSTANCES.lock().unwrap().is_empty() {
                INSTANCES.lock().unwrap().pop();
            }
            INSTANCES.lock().unwrap().push(instance.as_weak());
            // shows the window on the screen and maintains an extra strong reference
            // if not called, the instance is dropped and lost
            let _ = instance.show();
        }
    } else {
        //slint_interpreter::print_diagnostics(&compiler.diagnostics());
        result.print_diagnostics();
    }
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
    float_value: f64,
    string_value: *const c_char,
}
impl JRvalue {
    fn new_bool(b: bool) -> Self {
        debug!("JRvalue.new_bool");
        JRvalue {
            magic: JRMAGIC,
            rtype: CString::new("Bool").unwrap().into_raw(),
            int_value: b.into(),
            float_value: 0.0,
            string_value: CString::new("").unwrap().into_raw(),
        }
    }
    fn new_undefined() -> Self {
        debug!("JRvalue.new_undefined");
        JRvalue {
            magic: JRMAGIC,
            rtype: CString::new("Unknown").unwrap().into_raw(),
            int_value: 0,
            float_value: 0.0,
            string_value: CString::new("").unwrap().into_raw(),
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

impl From<JRvalue> for Value {
    fn from(rv: JRvalue) -> Self {
        if rv.magic == JRMAGIC {
            unsafe {
                let rv_cstr = CStr::from_ptr(rv.rtype);
                let rv_type: String = rv_cstr.to_string_lossy().into_owned();
                if rv_type == "Bool" {
                    debug!("Value::From<JRvalue>:rv_type is Bool {}",rv.int_value);
                    let bool_val: bool = rv.int_value != 0;
                    return Value::from(bool_val);
                }
                if rv_type == "Integer" {
                    debug!("Value::From<JRvalue>:rv_type is Integer {}",rv.int_value);
                    return Value::from(rv.int_value);
                }
                if rv_type == "Float" {
                    debug!("Value::From<JRvalue>:rv_type is Float {}",rv.float_value);
                    return Value::from(rv.float_value);
                }
                if rv_type == "String" {
                    let cs: SharedString = CStr::from_ptr(rv.string_value).to_string_lossy().into_owned().into();
                    debug!("Value::From<JRvalue>:rv_type is String {}",cs);
                    return Value::from(cs);
                }
                //if rv_type == "StandardListViewItem" {
                //    let cs: SharedString = CStr::from_ptr(rv.slvi_value).to_string_lossy().into_owned().into();
                //    debug!("Value::From<JRvalue>:rv_type is String {}",cs);
                //    return Value::from(cs);
                //}
                if rv_type == "Unknown" {
                    warn!("From<JRvalue>:can't set an unknown value type");
                }
            }
        }
        else {
            warn!("From<JRvalue>:not a valid JRvalue, JRvalue.magic must equal {}",JRMAGIC);
        }
        return Value::Void;
    }
}

//
// register a callback defined in .slint file
//   example line in .slint file:
//     pure callback validate-date(string) -> bool;
//   id is "validate-date" in this case
//   func is a C-callable function pointer
//
#[no_mangle]
pub unsafe extern "C" fn r_set_callback(id: *const c_char, func: extern "C" fn(par_ptr: *const c_void, len: i32) -> JRvalue ) {
    debug!("r_set_callback");
    let funcid: String = CStr::from_ptr(id).to_string_lossy().into_owned();
    if ! INSTANCES.lock().unwrap().is_empty() {
        //let ref i_ref = &(INSTANCES.lock().unwrap())[0];
        //let instance = i_ref.upgrade();
        let instance = (&(INSTANCES.lock().unwrap())[0]).upgrade();
        if instance.is_some() {
            let _ = instance.unwrap().set_callback(&funcid, move |args: &[Value]| {
                // debug list of arguments
                debug!("r_set_callback:slint calls callback with {} arguments", args.len());
                for arg in args {
                    print_type_of(arg);
                    let vt = arg.value_type();
                    debug!("r_set_callback:value type is: {:#?}", vt);
                }
                // debug end

                // create a void ptr to list of arguments which is send to Julias callback.
                let args_ptr: *const c_void = args as *const [Value] as *const c_void;
                // get number of arguments. This is send to Julia callback, too.
                let len: i32 = args.len().try_into().unwrap();

                // debug void ptr
                debug!("r_set_callback:void ptr adress is: {:p}", args_ptr);
                let args2: &[Value] = std::slice::from_raw_parts(args_ptr as *const Value, args.len());
                for arg in args2 {
                    print_type_of(arg);
                    let vt = arg.value_type();
                    debug!("r_set_callback:value type is: {:#?}", vt);
                }

                // call Julia callback and receive a JRvalue struct as return value
                let rv: JRvalue = func(args_ptr,len);

                // debug JRvalue returned
                print_type_of(&rv);
                debug!("r_set_callback:return value magic is: {}", rv.magic);
                debug!("r_set_callback:return value type is: {:p}", rv.rtype);
                debug!("r_set_callback:return value int_value is: {}", rv.int_value);
                debug!("r_set_callback:return value string_value is: {:p}", rv.string_value);
                // debug end

                // valid JRvalue only if magic == 123456
                if rv.magic == JRMAGIC {
                    // get the type of the return value
                    let rv_cstr = CStr::from_ptr(rv.rtype);
                    let rv_type: String = rv_cstr.to_string_lossy().into_owned();

                    debug!("r_set_callback:rv type is:{}", rv_type);

                    // create a Slint::Value from JRvalue as a valid Slint return value of a callback
                    if rv_type == "Bool" {
                        if rv.int_value == 1 {
                            return Value::from(true);
                        } else {
                            return Value::from(false);                        
                        }
                    } else {
                        error!("r_set_callback:callback return value of type {} is not implemented",rv_type);
                    }
                } else {
                    error!("r_set_callback:callback must return a valid JRvalue, JRvalue.magic must equal {}",JRMAGIC);
                }
                // Unvalid or not implemented JRvalue type, return an empty/void
                return Value::from(Value::Void);
            } );
        } else {
            warn!("r_set_callback:last slint instance dropped, call Slint.CompileFromFile or Slint.CompileFromString again");
        }
    } else {
        warn!("r_set_callback:no slint instance available, call Slint.CompileFromFile or Slint.CompileFromString");
    }
}

#[no_mangle]
pub extern "C" fn r_run() {
    debug!("r_run");
    if ! INSTANCES.lock().unwrap().is_empty() {
        //let v = INSTANCES.lock().unwrap();
        //let ref i = &v[0];
        //let instance = i.upgrade();
        let instance = (&(INSTANCES.lock().unwrap())[0]).upgrade();
        if instance.is_some() {
            instance.unwrap().run().unwrap();
        } else {
            warn!("r_run:last slint instance dropped, call Slint.CompileFromFile or Slint.CompileFromString again");
        }
    } else {
        warn!("r_run:no slint instance available, call Slint.CompileFromFile or Slint.CompileFromString");
    }
}

//
// args_ptr must be the ptr to the list of arguments, &[Value], sent to the Julia callback from r_set_callback (see above)
// len is the count of arguments in the list, this is needed to reconstruct the list from the ptr
// return the type as a string of the argument at index
//
#[no_mangle]
pub unsafe extern "C" fn r_get_value_type(args_ptr: *const c_void, len: i32, index: i32) -> *mut c_char {
    debug!("r_get_value_type");
    debug!("r_get_value_type:void ptr adress to the list of arguments is: {:p}",args_ptr);
    debug!("r_get_value_type:number of arguments in this list: {}",len);
    
    // reconstruct the list of arguments from the void ptr
    let args: &[Value] = std::slice::from_raw_parts(args_ptr as *const Value, len as usize);
    // get the value type of the argument at index
    let vt = args[index as usize].value_type();

    if vt == ValueType::String {
        let vt_s = String::from("String");
        debug!("r_get_value_type:value type of argument is {}", vt_s);
        let cstring = CString::new(vt_s).unwrap();
        return cstring.into_raw();
    } else if vt == ValueType::Number {
        let vt_s = String::from("Number");
        debug!("r_get_value_type:value type of argument is {}", vt_s);
        let cstring = CString::new(vt_s).unwrap();
        return cstring.into_raw();
    } else {
        warn!("r_get_value_type:argument type {:#?} is not yet implemented",vt);
    }
    // return an empty value type
    let cstring = CString::new("").unwrap();
    return cstring.into_raw();
}

//
// args_ptr must be the ptr to the list of arguments, &[Value], sent to the Julia callback from r_set_callback (see above)
// len is the count of arguments in the list, this is needed to reconstruct the list from the ptr
// return the value of the argument at index as a string
//
#[no_mangle]
pub unsafe extern "C" fn r_get_value_string(args_ptr: *const c_void, len: i32, index: i32) -> *mut c_char  {
    debug!("r_get_value_string");
    debug!("r_get_value_string:void ptr adress to the list of arguments is: {:p}",args_ptr);
    debug!("r_get_value_string:number of arguments in this list: {}",len);

    // reconstruct the list of arguments from the void ptr
    let args: &[Value] = std::slice::from_raw_parts(args_ptr as *const Value, len as usize);
    // check if argument is a string
    let vt = args[index as usize].value_type();
    if vt == ValueType::String {
        // get the arguments value
        let arg: SharedString = args[index as usize].clone().try_into().unwrap();
        // convert it to a Julia usable string type:
        let s: &str = arg.as_str();
        debug!("r_get_value_string:arguments value is: {}",s);
        let cstring = CString::new(s).unwrap();
        return cstring.into_raw();
    } else {
        warn!("r_get_value_string:argument type at index {} is not a string", index);
    }
    // return an empty value
    let cstring = CString::new("").unwrap();
    return cstring.into_raw();
}

//
// args_ptr must be the ptr to the list of arguments, &[Value], sent to the Julia callback from r_set_callback (see above)
// len is the count of arguments in the list, this is needed to reconstruct the list from the ptr
// return the value of the argument at index as a string
//
#[no_mangle]
pub unsafe extern "C" fn r_get_value_number(args_ptr: *const c_void, len: i32, index: i32, nan: f64) -> f64  {
    debug!("r_get_value_number");
    debug!("r_get_value_number:void ptr adress to the list of arguments is: {:p}",args_ptr);
    debug!("r_get_value_number:number of arguments in this list: {}",len);

    // reconstruct the list of arguments from the void ptr
    let args: &[Value] = std::slice::from_raw_parts(args_ptr as *const Value, len as usize);
    // check if argument is a string
    let vt = args[index as usize].value_type();
    if vt == ValueType::Number {
        // get the arguments value
        let arg: f64 = args[index as usize].clone().try_into().unwrap();
        debug!("r_get_value_number:arguments value is: {}",arg);
        return arg;
    } else {
        warn!("r_get_value_number:argument type at index {} is not a string", index);
    }
    // return NaN provided from caller
    return nan;
}

//
// API to models for arrays/matrices starts here
//    see array in https://slint.dev/releases/1.3.2/docs/rust/slint/docs/type_mappings/ 
//    https://slint.dev/releases/1.3.2/docs/rust/slint/trait.Model#
//    https://slint.dev/releases/1.3.2/docs/slint/src/language/syntax/repetitions
//
// an element of such an array is often called "cell"
//


use std::collections::HashMap;
use std::ptr;

// all models are stored here
static mut MODELS: Lazy<Mutex<HashMap<String,Rc<CellsModel>>>> = Lazy::new(|| {
    Mutex::new(HashMap::new())
});
unsafe fn model_contains(propertyid: &String) -> bool {
    //let model = MODELS.lock().unwrap();
    //return (*model).contains_key(&propertyid);
    let mod_ptr = ptr::addr_of_mut!(MODELS);
    return (*mod_ptr).lock().unwrap().contains_key(propertyid);
}
unsafe fn model_get(propertyid: &String) -> Rc<CellsModel> {
    let mod_ptr = ptr::addr_of_mut!(MODELS);
    return (*mod_ptr).lock().unwrap().get(propertyid).unwrap().clone();
}
unsafe fn model_insert(propertyid: String, model: Rc<CellsModel>) {
    let mod_ptr = ptr::addr_of_mut!(MODELS);
    (*mod_ptr).lock().unwrap().insert(propertyid,model);
}

// sometimes the update_cell callback should not be called, e.g. if changing a cell during update_cell
static SKIP_CALLBACK: Lazy<Mutex<bool>> = Lazy::new(|| {
    Mutex::new(false)
});
unsafe fn set_skip_callback(b: bool) {
    let mut skip = SKIP_CALLBACK.lock().unwrap();
    *skip = b;
}
unsafe fn get_skip_callback() -> bool {
    let skip = SKIP_CALLBACK.lock().unwrap();
    *skip
}

//
//
//
#[no_mangle]
pub unsafe extern "C" fn r_remove_row(id: *const c_char, index: usize) {
    debug!("r_pop_row");
    let propertyid: String = CStr::from_ptr(id).to_string_lossy().into_owned();
    //if ! MODELS.lock().unwrap().contains_key(&propertyid) {
    if ! model_contains(&propertyid) {
        warn!("r_pop_row:no model available for property id <{}>",propertyid);
    } else {
        debug!("r_pop_row: index: {}",index);
        //let model: Rc<CellsModel> = MODELS.lock().unwrap().get(&propertyid).unwrap().clone();
        let model: Rc<CellsModel> = model_get(&propertyid);
        model.remove_row(index);
    }
}

//
//
//
#[no_mangle]
pub unsafe extern "C" fn r_push_row(id: *const c_char, new_values: *const JRvalue, len: usize) {
    debug!("r_push_row");
    let propertyid: String = CStr::from_ptr(id).to_string_lossy().into_owned();
    //if ! MODELS.lock().unwrap().contains_key(&propertyid) {
    if ! model_contains(&propertyid) {
        warn!("r_push_row:no model available for property id <{}>",propertyid);
    } else {
        debug!("r_push_row: new_values size: {}",len);
        //let model: Rc<CellsModel> = MODELS.lock().unwrap().get(&propertyid).unwrap().clone();
        let model: Rc<CellsModel> = model_get(&propertyid);
        let row_count = model.row_count() + 1;
        let some_row = model.rows.borrow()[0].clone();

        let mut values: Vec<SlintValue> = Vec::new();
        let new_values_vec = std::slice::from_raw_parts(new_values as *const JRvalue, len);

        for index in 0..len {
            let value: JRvalue = new_values_vec[index];
            debug!("r_push_row: value.magic is {}",value.magic);
            if value.magic == JRMAGIC {
                let rv_cstr = CStr::from_ptr(value.rtype);
                let rv_type: String = rv_cstr.to_string_lossy().into_owned();
                debug!("r_push_row: rv_type is {}",rv_type);
                if rv_type == "Bool" {
                    let mut sv = SlintValue::default();
                    sv.value_i = value.int_value;
                    debug!("r_push_row: sv.value_i is {}",sv.value_i);
                    values.push(sv);
                }        
                if rv_type == "Integer" {
                    let mut sv = SlintValue::default();
                    sv.value_i = value.int_value;
                    debug!("r_push_row: sv.value_i is {}",sv.value_i);
                    values.push(sv);
                }        
                if rv_type == "Float" {
                    let mut sv = SlintValue::default();
                    sv.value_f = value.float_value;
                    debug!("r_push_row: sv.value_f is {}",sv.value_f);
                    values.push(sv);
                }        
                if rv_type == "String" {
                    let mut sv = SlintValue::default();
                    sv.value_s = CStr::from_ptr(value.string_value).to_string_lossy().into_owned().into();
                    debug!("r_push_row: sv.value_s is {}",sv.value_s);
                    values.push(sv);
                }        
                if rv_type == "Unknown" {
                    let mut sv = SlintValue::default();
                    sv.value_i = value.int_value;
                    sv.value_f = value.float_value;
                    sv.value_s = CStr::from_ptr(value.string_value).to_string_lossy().into_owned().into();
                    debug!("r_push_row: sv.value_i is {}",sv.value_i);
                    debug!("r_push_row: sv.value_f is {}",sv.value_f);
                    debug!("r_push_row: sv.value_s is {}",sv.value_s);
                    values.push(sv);
                }
            }
        }

        let _new_row = Rc::new(RowModel {
                row: row_count,
                row_elements: values.into(),
                base_model: some_row.base_model.clone(),
                notify: Default::default(),
                func: some_row.func,
            });
        
        model.push_row(_new_row);
        //set_skip_callback(true);
        //model.rows.borrow_mut().push(_new_row);
        //set_skip_callback(false);

        //model.rows.borrow()[0].notify.row_changed(0);
        //model.rows.notify.row_changed(row_count);

        //debug!("{}",model.row_count());
    }
}

//
// set the value of a property
//   the call_back is not called during this explicit update, as the caller already should know, that he updates the property
// 
#[no_mangle]
pub unsafe extern "C" fn r_set_value(id: *const c_char, new_value: JRvalue) {    
    debug!("r_set_value");
    let propertyid: String = CStr::from_ptr(id).to_string_lossy().into_owned();

    if ! INSTANCES.lock().unwrap().is_empty() {
        let instance = (&(INSTANCES.lock().unwrap())[0]).upgrade();
        if instance.is_some() {
            debug!("r_set_value: new_value.int_value={}",new_value.int_value);
            debug!("r_set_value: new_value.float_value={}",new_value.float_value);
            debug!("r_set_value: new_value.string_value={:p}",new_value.string_value);
            let _ = instance.unwrap().set_property(&propertyid, Value::from(new_value));
        } else {
            warn!("r_set_value:last slint instance dropped, call Slint.CompileFromFile or Slint.CompileFromString again");
        }
    } else {
        warn!("r_set_value:no slint instance available, call Slint.CompileFromFile or Slint.CompileFromString");
    }
}

//
// set the string value of a cell
//   the call_back is not called during this explicit update, as the caller already should know, that he updates the cell
// 
#[no_mangle]
//pub unsafe extern "C" fn r_set_cell_value(id: *const c_char, mut row: i32, mut col: i32, new_value: *const c_char) {
pub unsafe extern "C" fn r_set_cell_value(id: *const c_char, mut row: i32, mut col: i32, new_value: JRvalue) {    
    debug!("r_set_cell_value");
    if row == 0 {
        warn!("r_set_cell_value: row index is <{}>, please provide 1-based indices as in Julia",row);
    }
    if col == 0 {
        warn!("r_set_cell_value: column index is <{}>, please provide 1-based indices as in Julia",col);
    }
    row -= 1;
    col -= 1;
    let propertyid: String = CStr::from_ptr(id).to_string_lossy().into_owned();

    //if ! MODELS.lock().unwrap().contains_key(&propertyid) {
    if ! model_contains(&propertyid) {
        warn!("r_set_cell_value:no model available for property id <{}>",propertyid);
    } else {
        //let model: Rc<CellsModel> = MODELS.lock().unwrap().get(&propertyid).unwrap().clone();
        let model: Rc<CellsModel> = model_get(&propertyid);

        //let v: String = CStr::from_ptr(new_value.string_value).to_string_lossy().into_owned();
        //let shstr = slint_interpreter::SharedString::try_from(v).unwrap();

        set_skip_callback(true);
        model.update_cell(row as usize, col as usize, Some(new_value));
        set_skip_callback(false);
    }
}

//
// get the value of a cell as a string wrapped in a JRvalue struct
//
#[no_mangle]
//pub unsafe extern "C" fn r_get_cell_value(id: *const c_char, row: i32, col: i32) -> *mut c_char  {
pub unsafe extern "C" fn r_get_cell_value(id: *const c_char, mut row: i32, mut col: i32) -> JRvalue {
    debug!("r_get_cell_value");
    if row == 0 {
        warn!("r_get_cell_value: row index is <{}>, please provide 1-based indices as in Julia",row);
    }
    if col == 0 {
        warn!("r_get_cell_value: column index is <{}>, please provide 1-based indices as in Julia",col);
    }
    row -= 1;
    col -= 1;
    let propertyid: String = CStr::from_ptr(id).to_string_lossy().into_owned();

    let mut rv = JRvalue::new_undefined();

    //if ! MODELS.lock().unwrap().contains_key(&propertyid) {
    if ! model_contains(&propertyid) {
        warn!("r_get_cell_value:no model available for property id <{}>",propertyid);
    } else {
        //let model: Rc<CellsModel> = MODELS.lock().unwrap().get(&propertyid).unwrap().clone();
        let model: Rc<CellsModel> = model_get(&propertyid);
        //let v: Option<String> = model.get_cell_value(row as usize, col as usize);
        let rv_tmp: Option<JRvalue> = model.get_cell_value(row as usize, col as usize);
        //match v {
        match rv_tmp {
            Some(x) => {
                debug!("r_get_cell_value:cell value: {:p}",x.string_value);
                rv.rtype = x.rtype;
                rv.string_value = x.string_value;
                rv.int_value = x.int_value;
                rv.float_value = x.float_value;
            },
            None => debug!("r_get_cell_value:no cell value"),
        }
    }

    debug!("r_get_cell_value:return value: {}",rv.magic);
    let rv_cstr = CStr::from_ptr(rv.rtype);
    let rv_type: String = rv_cstr.to_string_lossy().into_owned();
    debug!("r_get_cell_value:return value type: {}",rv_type);
    debug!("r_get_cell_value:return value int: {}",rv.int_value);
    debug!("r_get_cell_value:return value float: {}",rv.float_value);
    debug!("r_get_cell_value:return value string_p: {:p}",rv.string_value);
    let cs: SharedString = CStr::from_ptr(rv.string_value).to_string_lossy().into_owned().into();
    debug!("r_get_cell_value:return value string: {}",cs);

    return rv;
}

//
// set the model for a slint vector property (id is the slint property id as string)
//   and register the callback for "update_cell", which is called when a cell value has changed
//
#[no_mangle]
pub unsafe extern "C" fn r_set_property_model(id: *const c_char, rows: i32, cols: i32, 
    func: Option<extern "C" fn(par_ptr: *const c_void, len: i32) -> JRvalue> 
) {
    debug!("r_set_property_model");
    let propertyid: String = CStr::from_ptr(id).to_string_lossy().into_owned();
    if ! INSTANCES.lock().unwrap().is_empty() {
        //let ref i_ref = &(INSTANCES.lock().unwrap())[0];
        //let instance = i_ref.upgrade();
        let instance = (&(INSTANCES.lock().unwrap())[0]).upgrade();
        if instance.is_some() {

            //test code start
            let v = instance.as_ref().unwrap().get_property(&propertyid);
            match v {
                Ok(value) => {
                    debug!("r_set_property_model:property <{}> has value: {:?}", propertyid, value);
                    print_type_of(&value);
                },
                Err(error) => warn!("r_set_property_model:getting property <{}> failed: {:?}", propertyid, error),
            };

            //test code end

            let model = CellsModel::new(rows as usize,cols as usize, func);
            //MODELS.lock().unwrap().insert(propertyid.clone(),model.clone());
            model_insert(propertyid.clone(),model.clone());
            let r = instance.unwrap().set_property(&propertyid,Value::Model(model.clone().into()));
            match r {
                Ok(_) => (),
                Err(error) => warn!("r_set_property_model:setting model for property <{}> failed: {:?}", propertyid, error),
            };
        } else {
            warn!("r_set_property_model:last slint instance dropped, call Slint.CompileFromFile or Slint.CompileFromString again");
        }
    } else {
        warn!("r_set_property_model:no slint instance available, call Slint.CompileFromFile or Slint.CompileFromString");
    }
}

//
// below the generic model for every slint 2-dimensional vector property
//   1-dimension vectors are handled like 2 dimensions with length 1 of one dimension
//   cell/element values are always strings
//
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
        //&()
        &self.notify
    }
}
extern "C" fn def_cb(_par_ptr: *const c_void, _len: i32) -> JRvalue {
    debug!("CellsModel.def_cb");
    JRvalue::new_undefined()
}
impl CellsModel {

    fn new( nrows: usize, ncols: usize, func: Option<extern "C" fn(par_ptr: *const c_void, len: i32) -> JRvalue> ) -> Rc<Self> {
        debug!("CellsModel.new");
        Rc::new_cyclic(|w| Self {
            rows: RefCell::new((0..nrows)
                .map(|row| {
                    Rc::new(RowModel {
                        row,
                        row_elements: vec![SlintValue::default(); ncols].into(),
                        base_model: w.clone(),
                        notify: Default::default(),
                        func,
                    })
                })
                .collect()),
            notify: Default::default(),
        })
    }

    fn push_row(&self, row: Rc<RowModel>) {
        debug!("CellsModel.push_row");
        self.rows.borrow_mut().push(row);
        let c = self.rows.borrow().len();
        self.notify.row_added(c-1,c);
    }

    fn remove_row(&self, index: usize ) {
        debug!("CellsModel.remove_row");
        if index > 0 && index < self.rows.borrow().len() {
            self.rows.borrow_mut().remove(index);
            let c = self.rows.borrow().len();
            self.notify.row_removed(index,c);
        } else {
            warn!("CellsModel.remove_row: trying to remove row index {} but length of rows is only {}",index,self.rows.borrow().len());
        }
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
        debug!("CellsModel.get_cell_value: row={} col={}",row,col);
        if row >= self.row_count() {
            warn!("CellsModel.get_cell_value: row index <{}> not in range of existing row indices <1..{}>",row,self.row_count());
        }
        if col >= self.col_count() {
            warn!("CellsModel.get_cell_value: col index <{}> not in range of existing column indices <1..{}>",col,self.col_count());
        }
        //let v: String = self.rows.get(row)?.row_elements.borrow().get(col)?.value_s.clone();
        let mut rv = JRvalue::new_undefined();
        //rv.string_value = self.rows.get(row)?.row_elements.borrow().get(col)?.value_s.clone();
        rv.int_value = self.rows.borrow().get(row)?.row_elements.borrow().get(col)?.value_i;
        rv.float_value = self.rows.borrow().get(row)?.row_elements.borrow().get(col)?.value_f;
        rv.string_value = CString::new(self.rows.borrow().get(row)?.row_elements.borrow().get(col)?.value_s.clone()).unwrap().into_raw();
        Some(rv)
    }

    //fn update_cell(&self, row: usize, col: usize, new_value: Option<SharedString>) -> Option<()> {
    fn update_cell(&self, row: usize, col: usize, new_value: Option<JRvalue>) -> Option<()> {
        debug!("CellsModel.update_cell");
        //debug!("CellsModel.update_cell: row={} col={} new_value={}",row,col,new_value.as_ref().unwrap());
        match new_value {
            Some(new_v) => {
                debug!("CellsModel.update_cell: row={} col={}",row,col);
                debug!("CellsModel.update_cell: new_v.int_value={}",new_v.int_value);
                debug!("CellsModel.update_cell: new_v.float_value={}",new_v.float_value);
                debug!("CellsModel.update_cell: new_v.string_value={:p}",new_v.string_value);
                if row >= self.row_count() {
                    warn!("CellsModel.update_cell: row index <{}> not in range of existing row indices <1..{}>",row,self.row_count());
                }
                if col >= self.col_count() {
                    warn!("CellsModel.update_cell: col index <{}> not in range of existing column indices <1..{}>",col,self.col_count());
                }
                let rows_tmp = self.rows.borrow();
                let r_model = rows_tmp.get(row)?;
                let mut row_el = r_model.row_elements.borrow_mut();
                let data = row_el.get_mut(col)?;

                let mut rv = JRvalue::new_bool(true);
                unsafe {
                    let args = &[
                        Value::Number((row as i32).into()),
                        Value::Number((col as i32).into()),
                        //Value::String(new_value.as_ref().unwrap().clone()),
                        Value::String(CStr::from_ptr(new_v.string_value).to_string_lossy().into_owned().into()),
                        Value::String(data.value_s.clone().into())
                        ];
            
                    drop(row_el);
                    
                    // create a void ptr to list of arguments which is send to Julias callback.
                    let args_ptr: *const c_void = args as *const [Value] as *const c_void;
                    // get number of arguments. This is send to Julia callback, too.
                    let len: i32 = args.len().try_into().unwrap();
            
                    // call Julia callback and receive a JRvalue struct as return value
                    if ! get_skip_callback() {
                        let f = r_model.func.unwrap_or(def_cb);
                        rv = (f)(args_ptr,len);
                    }
                }
        
                // debug JRvalue returned
                print_type_of(&rv);
                debug!("CellsModel.update_cell:return value magic is: {}", rv.magic);
                debug!("CellsModel.update_cell:return value type is: {:p}", rv.rtype);
                debug!("CellsModel.update_cell:return value int_value is: {}", rv.int_value);
                debug!("CellsModel.update_cell:return value float_value is: {}", rv.float_value);
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
        
                debug!("CellsModel.update_cell: data.value_i={}",data.value_i);
                debug!("CellsModel.update_cell: data.value_f={}",data.value_f);
                debug!("CellsModel.update_cell: data.value_s={}",data.value_s);

                // set the new value
                debug!("CellsModel.update_cell SET:");
                debug!("CellsModel.update_cell: new_v.int_value={}",new_v.int_value);
                debug!("CellsModel.update_cell: new_v.float_value={}",new_v.float_value);
                debug!("CellsModel.update_cell: new_v.string_value={:p}",new_v.string_value);
                unsafe {
                    data.value_s = CStr::from_ptr(new_v.string_value).to_string_lossy().into_owned();
                    data.value_i = new_v.int_value;
                    data.value_f = new_v.float_value;
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
    func: Option<extern "C" fn(par_ptr: *const c_void, len: i32) -> JRvalue>,
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
        debug!("RowModel.row_data: row={}",row);
        self.row_elements.borrow().get(row).map(|row_element| {
            debug!("RowModel.row_data: row_element.value_i={}",row_element.value_i);
            debug!("RowModel.row_data: row_element.value_f={}",row_element.value_f);
            debug!("RowModel.row_data: row_element.value_s={}",row_element.value_s);
            //debug!("RowModel.row_data: row_element.value_slvi={}",row_element.value_slvi.text);
            let mut stru = slint_interpreter::Struct::default();
            stru.set_field("value_i".into(), Value::Number(row_element.value_i.into()));
            stru.set_field("value_f".into(), Value::Number(row_element.value_f.into()));
            stru.set_field("value_s".into(), Value::String(row_element.value_s.clone().into()));
            //stru.set_field("value_slvi".into(), Value::String(row_element.value_slvi.text.clone().into()));
            stru.into()
        })
    }

    fn model_tracker(&self) -> &dyn ModelTracker {
        debug!("RowModel.model_tracker");
        &self.notify
    }

    fn set_row_data(&self, row: usize, data: Value) {
        debug!("RowModel.set_row_data");
        debug!("RowModel.set_row_data: row={} data.value_type={:#?}",row,data.value_type());
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













/*
struct RowModel {
    row: usize,
    row_elements: RefCell<Vec<SlintValue>>,
    base_model: std::rc::Weak<CellsModel>,
    notify: ModelNotify,
}

impl Model for RowModel {
    type Data = SlintValue;

    fn row_count(&self) -> usize {
        self.row_elements.borrow().len()
    }

    fn row_data(&self, row: usize) -> Option<Self::Data> {
        self.row_elements.borrow().get(row).cloned()
    }

    fn model_tracker(&self) -> &dyn ModelTracker {
        &self.notify
    }

    fn set_row_data(&self, index: usize, data: SlintValue) {
        if let Some(cells) = self.base_model.upgrade() {
        }
    }
}

struct CellsModel {
    rows: Vec<Rc<RowModel>>,
}
impl Model for CellsModel {
    type Data = ModelRc<SlintValue>;

    fn row_count(&self) -> usize {
        self.rows.len()
    }

    fn row_data(&self, row: usize) -> Option<Self::Data> {
        self.rows.get(row).map(|x| x.clone().into())
    }

    fn model_tracker(&self) -> &dyn ModelTracker {
        &()
    }
}
impl CellsModel {
    fn new(nrows: usize, ncols: usize) -> Rc<Self> {
        Rc::new_cyclic(|w| Self {
            rows: (0..(nrows-1))
                .map(|row| {
                    Rc::new(RowModel {
                        row,
                        row_elements: vec![SlintValue::default(); ncols].into(),
                        base_model: w.clone(),
                        notify: Default::default(),
                    })
                })
                .collect(),
        })
    }
}
*/


/*
#[no_mangle]
pub unsafe extern "C" fn r_run_from_file(cstring: *const c_char) {
    let cstr = CStr::from_ptr(cstring);
    let filename: String = cstr.to_string_lossy().into_owned();
    
    let mut compiler = ComponentCompiler::default();
    let definition =
        spin_on::spin_on(compiler.build_from_path(filename));
    if compiler.diagnostics().is_empty() {
        if let Some(definition) = definition {
            let instance = definition.create().unwrap();
            //let _ = instance.set_callback("button-clicked", |_| {TESTCALLBACK();Value::from(Value::Void)} );
            instance.run().unwrap();
        }
    } else {
        slint_interpreter::print_diagnostics(&compiler.diagnostics());
    }
}

#[no_mangle]
pub unsafe extern "C" fn r_run_from_string(cstring: *const c_char) {
    let cstr = CStr::from_ptr(cstring);
    let codestring: String = cstr.to_string_lossy().into_owned();
    let mut compiler = ComponentCompiler::default();
    let definition =
        spin_on::spin_on(compiler.build_from_source(codestring.into(), Default::default()));
    if compiler.diagnostics().is_empty() {
        let instance = definition.unwrap().create().unwrap();
        instance.run().unwrap();
    } else {
        slint_interpreter::print_diagnostics(&compiler.diagnostics());
    }
}
*/

/*
//pub unsafe extern "C" fn test_conv(shstr: SharedString ) -> *const u8 {
//pub unsafe extern "C" fn test_conv(shstr: *const u8 ) -> *mut c_char {    
//pub unsafe extern "C" fn test_conv(ptr: *const SharedString) -> CString {    
#[no_mangle]
pub unsafe extern "C" fn r_test_conv(ptr: *const c_void) -> *mut c_char {    
    println!("1");
    println!("{:p}",ptr);
    //unsafe {
    //    let ss: SharedString = (*ptr).clone();
    //    print_type_of(&ss);
    //    println!("{}",ss);
    //}
    let ss: SharedString = unsafe {
        assert!(!ptr.is_null());
        let ss_ptr: *const SharedString = ptr as *const SharedString;
        (*ss_ptr).clone()
    };
    println!("{}",ss);
    let str = ss.to_string();
    let c_str = CString::new(str).unwrap();
    c_str.into_raw()
    //return c_str;
    //print_type_of(ss);
    //println!("{}",ss);
//    let s = SharedString::new(shstr);
//   print_type_of(&shstr);
//    println!("{}",shstr);
//    let mystr: &str = shstr.borrow();
//    let mut string = mystr.to_string();
//    println!("{}",string);
    //let ptr = shstr.as_ptr();
    //let mut c_str = unsafe {
    //    CStr::from_ptr(ptr)
    //};
    //return c_str;
//    let c_str_song = CString::new(string).unwrap();
//    c_str_song.into_raw()
}
*/

    
//use std::os::raw::c_char;
//use std::os::raw::c_void;
//use std::ffi::CStr;

//use slint_interpreter::{ComponentDefinition, ComponentCompiler, Value, SharedString, ComponentHandle};
//use slint::{SharedString };

//use julia::api::{Julia, Value};

//use bevy::prelude::*;
//use bevy::window::WindowPlugin;
//use bevy::ui::UiPlugin;

//slint::slint!{
//    export global Logic {
//        pure callback button-clicked();
//        // You can collect other global properties here
//    }
//}


//use std::borrow::Borrow;
//use std::slice::from_raw_parts;

/*
#[no_mangle]
pub unsafe extern "C" fn set_callback_specific(cstring: *const c_char, func: extern fn(arg: *const c_void) -> i32 ) {
    let cstr = CStr::from_ptr(cstring);
    let funcid: String = cstr.to_string_lossy().into_owned();
    if ! INSTANCES.lock().unwrap().is_empty() {
        let v = INSTANCES.lock().unwrap();
        let ref i = &v[0];
        let instance = i.upgrade();
        if instance.is_some() {
            //instance.unwrap().set_callback(&funcid, move |_| {func();Value::from(Value::Void)} );
            let _ = instance.unwrap().set_callback(&funcid, move |args: &[Value]| {
                println!("set_callback");
                for arg in args {
                    print_type_of(arg);
                    let vt = arg.value_type();
                    println!("{:#?}", vt);
                }
                let arg: SharedString = args[0].clone().try_into().unwrap();
                let ptr = &arg as *const SharedString as *const c_void;
                println!("{}",arg);
                println!("{:p}",ptr);
                let r: i32 = func(ptr);
                println!("return: {}",r);
                if r == 1 {
                    return true.into();
                }
                //Value::from(Value::Void)
                false.into()
            } );
            //instance.unwrap().set_callback("validate-date", move |args: &[Value]| {let arg: SharedString = args[0].clone().try_into().unwrap();println!("{}",arg);test_callback1(arg);func();Value::from(Value::Void)} );
        }
    }
}
*/

/*
fn test_callback1(shstr: SharedString) {
    println!("Button clicked");
    println!("{}",shstr);
    print_type_of(&shstr);
    let mystr: &str = shstr.borrow();
    println!("{}",mystr);
    let string = mystr.to_string();
    println!("{}",string);
}

fn test_callback2(ptr: *const str) {
    println!("Button clicked");
    println!("{:p}",ptr);
    //let _ss = unsafe {
    let _ss = {
        //println!("{}",*ptr);
        assert!(!ptr.is_null());
        &ptr
    };
    //print_type_of(ss);
}
 */


/* JUST SOME RANDOM TEST FUNCTIONS */
/*
static mut TESTCALLBACK: extern fn() = testinit;
extern "C" fn testinit() {
    panic!("Function pointer not initialized");
}
#[no_mangle]
pub extern "C" fn bw_test_set_callback( func: extern fn() ) {
    //let func_box: Box<extern fn()> = Box::new(func);
    //let func_ptr: *mut extern fn() = Box::into_raw(func_box);
    unsafe {
        //TESTCALLBACK = *func_ptr;
        TESTCALLBACK = func;
    }
    //println!("hello world");
}
#[no_mangle]
pub extern "C" fn bw_test_run_callback() {
    unsafe {
        TESTCALLBACK();
    }
    //println!("hello world");
}

fn hello_world() {
    println!("hello world!");
}

static mut CALLBACK: extern fn() = init;
extern "C" fn init() {
    panic!("Function pointer not initialized");
}
fn call_callback() {
    unsafe {
        CALLBACK();
    }
    //println!("hello world");
}

#[no_mangle]
pub extern "C" fn bw_app_add_system(app_ptr: *mut App, func: extern fn() ) {
    unsafe {
        CALLBACK = func;
        (*app_ptr).add_system(call_callback);
    }
}

/* ORIGINAL EXAMPLES FROM https://github.com/felipenoris/JuliaPackageWithRustDep.jl */

#[no_mangle]
pub extern "C" fn printhello() {
    println!("Hello from Rust!");
}

#[no_mangle]
pub extern "C" fn abs_i32(i: i32) -> i32 {
    println!("Rust read i32 `{:?}`.", i);
    if i >= 0 {
        i
    } else {
        -i
    }
}

#[no_mangle]
pub extern "C" fn abs_i64(i: i64) -> i64 {
    println!("Rust read i64 `{:?}`.", i);
    if i >= 0 {
        i
    } else {
        -i
    }
}

#[no_mangle]
pub extern "C" fn abs_f32(i: f32) -> f32 {
    println!("Rust read f32 `{:?}`.", i);
    if i >= 0.0 {
        i
    } else {
        -i
    }
}

#[no_mangle]
pub extern "C" fn abs_f64(i: f64) -> f64 {
    println!("Rust read f64 `{:?}`.", i);
    if i >= 0.0 {
        i
    } else {
        -i
    }
}

#[no_mangle]
pub extern "C" fn is_true_bool(b: bool) -> bool {
    println!("Rust read bool `{:?}`.", b);
    b
}

#[no_mangle]
pub unsafe extern "C" fn inspect_string(cstring: *const c_char) {
    let cstr = CStr::from_ptr(cstring);

    match cstr.to_str() {
        Ok(s) => {
            // `s` is a regular `&str`
            println!("Rust read `{:?}`.", s);
        }
        Err(_) => {
            panic!("Couldn't convert foreign Cstring to &str.");
        }
    }
}

#[no_mangle]
pub extern "C" fn generate_rust_owned_string() -> *mut c_char {
    let rust_string = String::from("The bomb: 💣");
    let cstring = CString::new(rust_string).unwrap();
    cstring.into_raw() // transfers ownership to the Julia process
}

#[no_mangle]
pub unsafe extern "C" fn free_rust_owned_string(s: *mut c_char) {
    if !s.is_null() {
        drop(CString::from_raw(s)); // retakes ownership of the CString and drops
    }
}
*/