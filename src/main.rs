#![crate_type="staticlib"]
#![feature(intrinsics)]
#![feature(no_std)]
#![feature(lang_items)]
#![feature(link_args)]
#![feature(alloc)]

#![no_std]

extern crate pebble;
extern crate alloc;

use pebble::types::{GPoint,GSize,GRect,ClickRecognizer,Window,WindowHandlers};
use pebble::types;
use pebble::raw;
use pebble::Layer;
use alloc::boxed::Box;

use core::prelude::*;


const CIRCLE_RADIUS: u16 = 12;
const CIRCLE_LINE_THICKNESS: u16 = 2;
const CIRCLE_PADDING: u16 = 14 - CIRCLE_RADIUS;
const CELL_SIZE: u16 = 2 * (CIRCLE_RADIUS + CIRCLE_PADDING);
const SIDE_PADDING: u16 = (144 - (4*CELL_SIZE))/2;
const TOP_PADDING: u16 = 0;

const CELLS_PER_ROW: u16 = 4;
const CELLS_PER_COLUMN: u16 = 6;

const HOURS_FIRST_DIGIT_ROW: u16 = 0;
const HOURS_SECOND_DIGIT_ROW: u16 = 1;
const MINUTES_FIRST_DIGIT_ROW: u16 = 2;
const MINUTES_SECOND_DIGIT_ROW: u16 = 3;
const SECONDS_FIRST_DIGIT_ROW: u16 = 4;
const SECONDS_SECOND_DIGIT_ROW: u16 = 5;

const DEFAULT_MAX_COLS: u16 = 4;
const HOURS_FIRST_DIGIT_MAX_COLS: u16 = 2;
const MINUTES_FIRST_DIGIT_MAX_COLS: u16 = 3;
const SECONDS_FIRST_DIGIT_MAX_COLS: u16 = 3;

static mut display_layer: *mut types::Layer = 0 as *mut types::Layer;

fn draw_cell(ctx: *mut types::GContext, center: GPoint, bit: u16) {
    raw::app_log("drawing cell\0"); 
    raw::graphics_context_set_fill_color(ctx, types::GColor::GColorWhite);
    raw::graphics_fill_circle(ctx, center, CIRCLE_RADIUS);

    if bit == 0 {
        raw::graphics_context_set_fill_color(ctx, types::GColor::GColorBlack);
        raw::graphics_fill_circle(ctx, center, CIRCLE_RADIUS-CIRCLE_LINE_THICKNESS);
    }
}

fn get_center_point_from_cell_location(x: u16, y: u16) -> GPoint {
    GPoint {
        x: SIDE_PADDING + (CELL_SIZE / 2) + (CELL_SIZE * x),
        y: TOP_PADDING + (CELL_SIZE / 2) + (CELL_SIZE * y)
    }
}

fn draw_cell_row_for_digit(ctx: *mut types::GContext, digit: u16, columns: u16, row: u16) {
    raw::app_log("drawing row\0");
    for i in 0..columns {
        raw::app_log("drawing loop\0");
        draw_cell(ctx, get_center_point_from_cell_location(i, row), (digit >> i) & 0x1);
    }
}

fn get_display_hour(hour: u32) -> u16 {
    raw::app_log("get hour\0");
    if raw::clock_is_24h_style() {
        return hour as u16;
    } 
    let display_hour = hour % 12;
    if display_hour > 0 { display_hour as u16 } else { 12 }
}

extern fn display_layer_update(layer: *mut types::Layer, ctx: *mut types::GContext) {
    raw::app_log("update\0");
    let now = raw::time();
    raw::app_log("update\0");
    let t = raw::localtime(now);
    raw::app_log("update\0");

    let mut min = 0;
    let mut sec = 0;
    let mut hr = 0;
    unsafe {
        hr = get_display_hour((*t).tm_hour);
        min = (*t).tm_min as u16;
        sec = (*t).tm_sec as u16;
    }
    raw::app_log("drawing cells\0");
    draw_cell_row_for_digit(ctx, hr / 10 , HOURS_FIRST_DIGIT_MAX_COLS, HOURS_FIRST_DIGIT_ROW);
    draw_cell_row_for_digit(ctx, hr % 10 , DEFAULT_MAX_COLS, HOURS_SECOND_DIGIT_ROW);

    raw::app_log("drawing cells\0");
    draw_cell_row_for_digit(ctx, min / 10, MINUTES_FIRST_DIGIT_MAX_COLS, MINUTES_FIRST_DIGIT_ROW);
    draw_cell_row_for_digit(ctx, min % 10, DEFAULT_MAX_COLS, MINUTES_SECOND_DIGIT_ROW);

    raw::app_log("drawing cells\0");
    draw_cell_row_for_digit(ctx, sec / 10, SECONDS_FIRST_DIGIT_MAX_COLS, SECONDS_FIRST_DIGIT_ROW);
    draw_cell_row_for_digit(ctx, sec % 10, DEFAULT_MAX_COLS, SECONDS_SECOND_DIGIT_ROW);
    raw::app_log("exiting update\0");
}

extern fn handle_second_tick(tick_time: *mut types::TM, units: types::TimeUnits) {
    raw::app_log("yo second\0");
    unsafe { 
        if display_layer != 0 as *mut types::Layer {
            raw::app_log("something here\0");
            raw::layer_mark_dirty(display_layer);
        } else {
            raw::app_log("nothing here\0");
        }
    }
}

extern fn window_load_handler(window: *mut Window) {
    let window_layer = raw::window_get_root_layer(window);
    let window_bounds = raw::layer_get_frame(window_layer);

    let display: *mut types::Layer = raw::layer_create(window_bounds);
    unsafe { display_layer = display; }

    raw::layer_set_update_proc(display, display_layer_update);
    raw::layer_add_child(window_layer, display);


    raw::tick_timer_service_subscribe(types::TimeUnits::SECOND_UNIT, handle_second_tick);
}

extern fn window_unload_handler(window: *mut Window) {
}
extern fn window_appear_handler(window: *mut Window) {
}
extern fn window_disappear_handler(window: *mut Window) {
    //unsafe { raw::layer_destroy(display_layer.unwrap()) }
}

fn init() -> *mut Window {
    let window = raw::window_create();
    raw::window_set_background_color(window, types::GColor::GColorBlack);
    raw::window_set_window_handlers(window, WindowHandlers {
        load: window_load_handler,
        appear: window_appear_handler,
        disappear: window_disappear_handler,
        unload: window_unload_handler,
    });

    raw::window_stack_push(window, true);
    window
}

fn deinit(window: *mut Window) {
    raw::window_destroy(window);
}

#[no_mangle]
pub extern fn main() {
    let window = init();
    raw::app_event_loop();
    deinit(window);
}
