use ratatui::Frame;
use ratatui::layout::Rect;

pub enum BlockingWidgetState<R> {
    NotQueued,
    Queued,
    Completed(R)
}

pub struct BlockingWidget<T: BlockingWidgetTrait<R>, R> {
    inner: T,
    state: BlockingWidgetState<R>
}

pub trait BlockingWidgetTrait<R> {
    fn render_preblock(&mut self, frame: &mut Frame, area: Rect);
    fn run_task_blocking() -> R;
}