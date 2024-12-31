use std::ffi::CString;

use ris_error::Extensions;
use ris_error::RisResult;
use ris_log::log_level::LogLevel;
use ris_log::log_message::LogMessage;

use crate::ui_helper::IUiHelperModule;
use crate::ui_helper::SharedStateWeakPtr;
use crate::ui_helper::UiHelperDrawData;

pub struct LogModule {
    log_level: LogLevel,
    filter: String,
    log_plain: bool,
}

impl IUiHelperModule for LogModule {
    fn name() -> &'static str {
        "log"
    }

    fn build(_shared_state: SharedStateWeakPtr) -> Box<dyn IUiHelperModule> {
        Box::new(Self {
            log_level: LogLevel::Debug,
            filter: String::new(),
            log_plain: false,
        })
    }

    fn draw(&mut self, data: &mut UiHelperDrawData) -> RisResult<()> {
        if data.ui.button("clear") {
            let mutex = &crate::log_appenders::ui_helper_appender::MESSAGES;
            let mut mutex_guard = mutex.lock()?;
            let messages = mutex_guard.as_mut().into_ris_error()?;
            messages.clear();
        }

        let label = CString::new("##log_level")?;
        let label_ptr = label.as_ptr();
        let mut current_item = usize::from(self.log_level) as i32;
        let items = [
            CString::new(format!("{:?}", LogLevel::Trace))?,
            CString::new(format!("{:?}", LogLevel::Debug))?,
            CString::new(format!("{:?}", LogLevel::Info))?,
            CString::new(format!("{:?}", LogLevel::Warning))?,
            CString::new(format!("{:?}", LogLevel::Error))?,
            CString::new(format!("{:?}", LogLevel::Fatal))?,
            CString::new(format!("{:?}", LogLevel::None))?,
        ];
        let item_ptrs = items.iter().map(|x| x.as_ptr()).collect::<Vec<_>>();
        let items_ptrs_ptr = item_ptrs.as_ptr();

        data.ui.same_line();
        //data.ui.combo_simple_string("##log_level", &mut current_item, &items);

        unsafe {
            imgui::sys::igSetNextItemWidth(80.0);
            imgui::sys::igCombo_Str_arr(
                label_ptr,
                &mut current_item,
                items_ptrs_ptr,
                item_ptrs.len() as i32,
                -1,
            )
        };
        self.log_level = LogLevel::from(current_item as usize);

        data.ui.same_line();
        data.ui.checkbox("plain logs", &mut self.log_plain);

        data.ui.same_line();
        data.ui.input_text("filter", &mut self.filter).build();

        data.ui.separator();

        let result = data
            .ui
            .child_window("log_scrolling")
            .build(|| self.draw_child(data));

        match result {
            Some(Err(e)) => Err(e),
            _ => Ok(()),
        }
    }
}

impl LogModule {
    fn draw_child(&mut self, data: &mut UiHelperDrawData) -> RisResult<()> {
        let mutex = &crate::log_appenders::ui_helper_appender::MESSAGES;
        let mut mutex_guard = mutex.lock()?;
        let messages = mutex_guard.as_mut().into_ris_error()?;

        for message in messages.iter() {
            match message {
                LogMessage::Plain(_plain) => {
                    if !self.log_plain {
                        continue;
                    }
                }
                LogMessage::Constructed(constructed) => {
                    if constructed.priority < self.log_level {
                        continue;
                    }
                }
            };

            let formatted_message = message.fmt(false);
            if !formatted_message
                .to_lowercase()
                .contains(&self.filter.to_lowercase())
            {
                continue;
            }

            data.ui.text(formatted_message);

            data.ui.separator();
        }

        if data.ui.scroll_y() >= data.ui.scroll_max_y() {
            unsafe { imgui::sys::igSetScrollHereY(1.0) };
        }

        Ok(())
    }
}
