//! System definitions
use legion::system;
use crate::on_event;

#[on_event(tick)]
#[system]
pub fn tick_test() {

}