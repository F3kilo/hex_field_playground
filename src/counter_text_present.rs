use crate::app::Present;
use crate::counter_model::CounterModel;
use crate::term_render::TermRender;

pub struct CounterTextPresent;

impl Present for CounterTextPresent {
    type Mod = CounterModel;
    type Rend = TermRender;

    fn present(&mut self, model: &Self::Mod, render: &mut Self::Rend) {
        render.add_line(format!("Values is: {}", model.value()).as_str())
    }
}
