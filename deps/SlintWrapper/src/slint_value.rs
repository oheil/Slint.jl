use slint::StandardListViewItem;
//use slint::SharedString;
use log::*;

//
// SlintValue is the central value type for all Slint models
//      see CellsModel and RowModel below
//      JRvalue is the corresponding type to Julia
//
#[derive(Clone)]
pub struct SlintValue {
    pub value_s: String,
    pub value_i: i32,
    pub value_f: f64,
    pub value_slvi: StandardListViewItem,
}

impl Default for SlintValue {
    fn default() -> SlintValue {
        debug!("SlintValue: default");
        SlintValue{
            value_s: String::from(""),
            value_i: 0,
            value_f: 0.0,
            value_slvi: StandardListViewItem::from(""),
        }
    }
}

/*
impl From<SharedString> for SlintValue {
    fn from(item: SharedString) -> Self {
        debug!("SlintValue: from -> SharedString");
        let s = item.to_string();
        let mut sv = SlintValue::default();
        sv.value_slvi = s;
        sv
    }
}
*/

impl From<SlintValue> for StandardListViewItem {
    fn from(item: SlintValue) -> Self {
        debug!("SlintValue: from -> StandardListViewItem");
        item.value_slvi
    }
}

impl From<StandardListViewItem> for SlintValue {
    fn from(item: StandardListViewItem) -> Self {
        debug!("StandardListViewItem: from -> SlintValue");
        let mut sv = SlintValue::default();
        sv.value_slvi = item;
        sv
    }
}

/*
pub trait VecInto<StandardListViewItem> {
    fn vec_into(self) -> Vec<StandardListViewItem>;
}
impl<SlintValue, StandardListViewItem> VecInto<StandardListViewItem> for Vec<SlintValue>
where
StandardListViewItem: From<SlintValue>,
{
  fn vec_into(self) -> Vec<StandardListViewItem> {
    debug!("SlintValue: Vec<SlintValue> -> Vec<StandardListViewItem>");
    self.into_iter().map(std::convert::Into::into).collect()
  }
}
 */

pub trait VecInto<SlintValue> {
    fn vec_into(self) -> Vec<SlintValue>;
}
impl<StandardListViewItem, SlintValue> VecInto<SlintValue> for Vec<StandardListViewItem>
where
StandardListViewItem: Into<SlintValue>,
{
  fn vec_into(self) -> Vec<SlintValue> {
    debug!("StandardListViewItem: Vec<StandardListViewItem> -> Vec<SlintValue>");
    self.into_iter().map(std::convert::Into::into).collect()
  }
}




/*
fn slint_value_list_2_standard_list_view_item_list( sv: &Vec<Vec<SlintValue>>, slvi: &mut Vec<StandardListViewItem> ) {
    debug!("slint_value_list_2_standard_list_view_item_list");
    // convert [[SlintValue]] to [StandardListViewItem]
    slvi.clear();
    for i in 0..sv[0].len() {
        let item = &sv[0][i];
        let slvi_item = StandardListViewItem::from(item.clone());
        slvi.push(slvi_item);
    }
}
 */

/*
pub fn convert<SlintValue, StandardListViewItem>(vector: Vec<SlintValue>) -> Vec<StandardListViewItem>
  where
    SlintValue: TryInto<StandardListViewItem>,
    <SlintValue as std::convert::TryInto<StandardListViewItem>>::Error: std::fmt::Display
{
	vector
        .into_iter()
        .map(|value_t| {
            match TryInto::try_into(value_t) {
                Ok(value_u) => value_u,
                Err(why) => {
                    let t = std::any::type_name::<SlintValue>();
					let u = std::any::type_name::<StandardListViewItem>();
					panic!("Error converting from {t} to {u}: {why}")
				}
			}
        }
    ).collect()
}
 */