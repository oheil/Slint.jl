
// set RUST_LOG=debug

use log::*;
use env_logger::Env;

//use slint_interpreter::{Weak, Value, ValueType, ComponentCompiler, ComponentInstance, ComponentHandle, SharedString};
use slint_interpreter::ComponentCompiler;
use slint_interpreter::ComponentHandle;
use slint_interpreter::Value;

pub fn main() {
    let env = Env::default()
        .filter_or("RUST_LOG", "info");
    env_logger::init_from_env(env);

    let mut compiler = ComponentCompiler::default();

    let code = r#"
    // Copyright © SixtyFPS GmbH <info@slint.dev>
    // SPDX-License-Identifier: MIT
    
    import { Button, LineEdit, ScrollView, GridBox} from "std-widgets.slint";
    
    struct SlintValue  { value: string }
    
    export component MainWindow inherits Window {
        width: 500px;
        height: 500px;

        in-out property <[SlintValue]> _row1: [{}, {},];
        in-out property <[SlintValue]> _row2: [{}, {},];
        in property <[[SlintValue]]> cells: [
            root._row1, root._row2,
        ];

        //in property <[[SlintValue]]> cells;
    
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
                            text: " " + cell.value;
                            vertical-alignment: center;
                            width: 100%;
                            height: 100%;
                        }
        
                        TouchArea {
                            clicked => {
                                l.text = cell.value;
                                root.active-cell = {r: row-idx, c: col-idx};
                                l.focus();
                            }
                        }
        
                        l := LineEdit {
                            edited => {
                                cell = { value: self.text };
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
    
    //export component Cell inherits MainWindow {
    //    // initialize the cells with demy value to be viewed in the preview
    //    in-out property <[SlintValue]> _row1: [{}, {},];
    //    in-out property <[SlintValue]> _row2: [{}, {},];
    //    cells: [
    //        root._row1, root._row2,
    //    ];
    //}

    "#;

    let definition = spin_on::spin_on(
        compiler.build_from_source(code.into(), Default::default()));

    slint_interpreter::print_diagnostics(&compiler.diagnostics());

    let instance = definition.unwrap().create().unwrap();

    let instance_weak = instance.as_weak();
    let _ = instance.set_callback("add_row",move |_| {
        let cells = instance_weak.unwrap().get_property("cells").unwrap();

        debug!("on_add_row {:#?}",cells);


        return Value::from(Value::Void);
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