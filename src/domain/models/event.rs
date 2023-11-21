use tui_textarea::Input;

use super::BackendResponse;
use super::Message;

pub enum Event {
    BackendMessage(Message),
    BackendPromptResponse(BackendResponse),
    KeyboardCharInput(Input),
    KeyboardCTRLC(),
    KeyboardCTRLR(),
    KeyboardEnter(),
    UIResize(),
    UIScrollDown(),
    UIScrollUp(),
    UIScrollPageDown(),
    UIScrollPageUp(),
}
