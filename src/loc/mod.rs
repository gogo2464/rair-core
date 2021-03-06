/*
 * loc: commands handling file location.
 * Copyright (C) 2019  Oddcoder
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */
mod history;
mod mode;
mod seek;
use self::history::History;
use self::mode::*;
use self::seek::*;
use crate::core::Core;
use parking_lot::Mutex;
use std::sync::Arc;

pub fn register_loc(core: &mut Core) {
    let history = Arc::new(Mutex::new(History::new()));
    core.add_command("mode", "m", Arc::new(Mutex::new(Mode::with_history(history.clone()))));
    core.add_command("seek", "s", Arc::new(Mutex::new(Seek::with_history(history))));
}
