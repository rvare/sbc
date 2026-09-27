/*
Copyright 2026 Richard Varela
This file is part of sca.

This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
*/

use sca;
use std::env;
use std::process;
use std::thread;

fn main() {
    let mut args_iter = env::args().skip(1);

    let available_threads: usize = match thread::available_parallelism() {
        Ok(non_zero) => non_zero.get(),
        Err(why) => {
            eprintln!("{}", why);
            process::exit(1);
        }
    };

    let params = sca::parse_cmd_parameters(&mut args_iter, available_threads);
    let counter = sca::process_source(params);

    println!("Number of statements: {}", counter.num_statements);
    println!("Number of blocks: {}", counter.num_blocks);
}
