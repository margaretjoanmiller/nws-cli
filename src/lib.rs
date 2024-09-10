// nws-cli, a CLI application that gets the forecast or current conditions from the National Weather Service
//     Copyright (C) 2024 Margaret Joan Miller

//     This program is free software: you can redistribute it and/or modify
//     it under the terms of the GNU General Public License as published by
//     the Free Software Foundation, either version 3 of the License, or
//     (at your option) any later version.

//     This program is distributed in the hope that it will be useful,
//     but WITHOUT ANY WARRANTY; without even the implied warranty of
//     MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//     GNU General Public License for more details.

//     You should have received a copy of the GNU General Public License
//     along with this program.  If not, see <https://www.gnu.org/licenses/>

// extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;


pub mod geocode;
pub mod grid_json;
pub mod forecast;
pub mod forecast_json;

