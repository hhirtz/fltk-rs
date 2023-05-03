/* automatically generated by rust-bindgen 0.64.0 */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Widget {
    _unused: [u8; 0],
}
pub type Fl_Callback = ::core::option::Option<
    unsafe extern "C" fn(arg1: *mut Fl_Widget, arg2: *mut ::std::os::raw::c_void),
>;
pub type custom_handler_callback = ::core::option::Option<
    unsafe extern "C" fn(
        arg1: *mut Fl_Widget,
        arg2: ::std::os::raw::c_int,
        arg3: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int,
>;
pub type custom_draw_callback = ::core::option::Option<
    unsafe extern "C" fn(arg1: *mut Fl_Widget, arg2: *mut ::std::os::raw::c_void),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Box {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_Box_new(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        title: *const ::std::os::raw::c_char,
    ) -> *mut Fl_Box;
}
extern "C" {
    pub fn Fl_Box_x(arg1: *mut Fl_Box) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Box_y(arg1: *mut Fl_Box) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Box_width(arg1: *mut Fl_Box) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Box_height(arg1: *mut Fl_Box) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Box_label(arg1: *mut Fl_Box) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Box_set_label(arg1: *mut Fl_Box, title: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn Fl_Box_redraw(arg1: *mut Fl_Box);
}
extern "C" {
    pub fn Fl_Box_show(arg1: *mut Fl_Box);
}
extern "C" {
    pub fn Fl_Box_hide(arg1: *mut Fl_Box);
}
extern "C" {
    pub fn Fl_Box_activate(arg1: *mut Fl_Box);
}
extern "C" {
    pub fn Fl_Box_deactivate(arg1: *mut Fl_Box);
}
extern "C" {
    pub fn Fl_Box_redraw_label(arg1: *mut Fl_Box);
}
extern "C" {
    pub fn Fl_Box_resize(
        arg1: *mut Fl_Box,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Box_widget_resize(
        arg1: *mut Fl_Box,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Box_tooltip(arg1: *mut Fl_Box) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Box_set_tooltip(arg1: *mut Fl_Box, txt: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn Fl_Box_get_type(arg1: *mut Fl_Box) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Box_set_type(arg1: *mut Fl_Box, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Box_color(arg1: *mut Fl_Box) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Box_set_color(arg1: *mut Fl_Box, color: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn Fl_Box_measure_label(
        arg1: *const Fl_Box,
        arg2: *mut ::std::os::raw::c_int,
        arg3: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Box_label_color(arg1: *mut Fl_Box) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Box_set_label_color(arg1: *mut Fl_Box, color: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn Fl_Box_label_font(arg1: *mut Fl_Box) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Box_set_label_font(arg1: *mut Fl_Box, font: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Box_label_size(arg1: *mut Fl_Box) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Box_set_label_size(arg1: *mut Fl_Box, sz: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Box_label_type(arg1: *mut Fl_Box) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Box_set_label_type(arg1: *mut Fl_Box, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Box_box(arg1: *mut Fl_Box) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Box_set_box(arg1: *mut Fl_Box, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Box_changed(arg1: *mut Fl_Box) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Box_set_changed(arg1: *mut Fl_Box);
}
extern "C" {
    pub fn Fl_Box_clear_changed(arg1: *mut Fl_Box);
}
extern "C" {
    pub fn Fl_Box_align(arg1: *mut Fl_Box) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Box_set_align(arg1: *mut Fl_Box, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Box_delete(arg1: *mut Fl_Box);
}
extern "C" {
    pub fn Fl_Box_set_image(arg1: *mut Fl_Box, arg2: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn Fl_Box_handle(
        self_: *mut Fl_Box,
        cb: custom_handler_callback,
        data: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn Fl_Box_handle_event(self_: *mut Fl_Box, event: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Box_draw(
        self_: *mut Fl_Box,
        cb: custom_draw_callback,
        data: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn Fl_Box_resize_callback(
        self_: *mut Fl_Box,
        cb: ::core::option::Option<
            unsafe extern "C" fn(
                arg1: *mut Fl_Widget,
                x: ::std::os::raw::c_int,
                y: ::std::os::raw::c_int,
                w: ::std::os::raw::c_int,
                h: ::std::os::raw::c_int,
                arg2: *mut ::std::os::raw::c_void,
            ),
        >,
        data: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn Fl_Box_set_when(arg1: *mut Fl_Box, arg2: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Box_when(arg1: *const Fl_Box) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Box_image(arg1: *const Fl_Box) -> *const ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Box_parent(self_: *const Fl_Box) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Box_selection_color(arg1: *mut Fl_Box) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Box_set_selection_color(arg1: *mut Fl_Box, color: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn Fl_Box_do_callback(arg1: *mut Fl_Box);
}
extern "C" {
    pub fn Fl_Box_inside(
        self_: *const Fl_Box,
        arg1: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Box_window(arg1: *const Fl_Box) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Box_top_window(arg1: *const Fl_Box) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Box_takes_events(arg1: *const Fl_Box) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Box_user_data(arg1: *const Fl_Box) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Box_take_focus(self_: *mut Fl_Box) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Box_set_visible_focus(self_: *mut Fl_Box);
}
extern "C" {
    pub fn Fl_Box_clear_visible_focus(self_: *mut Fl_Box);
}
extern "C" {
    pub fn Fl_Box_visible_focus(self_: *mut Fl_Box, v: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Box_has_visible_focus(self_: *mut Fl_Box) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Box_set_user_data(arg1: *mut Fl_Box, data: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn Fl_Box_draw_data(self_: *const Fl_Box) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Box_handle_data(self_: *const Fl_Box) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Box_set_draw_data(self_: *mut Fl_Box, data: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn Fl_Box_set_handle_data(self_: *mut Fl_Box, data: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn Fl_Box_damage(self_: *const Fl_Box) -> ::std::os::raw::c_uchar;
}
extern "C" {
    pub fn Fl_Box_set_damage(self_: *mut Fl_Box, flag: ::std::os::raw::c_uchar);
}
extern "C" {
    pub fn Fl_Box_set_damage_area(
        self_: *mut Fl_Box,
        flag: ::std::os::raw::c_uchar,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        w: ::std::os::raw::c_int,
        h: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Box_clear_damage(self_: *mut Fl_Box);
}
extern "C" {
    pub fn Fl_Box_as_window(self_: *mut Fl_Box) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Box_as_group(self_: *mut Fl_Box) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Box_set_deimage(arg1: *mut Fl_Box, arg2: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn Fl_Box_deimage(arg1: *const Fl_Box) -> *const ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Box_set_callback(
        arg1: *mut Fl_Box,
        arg2: Fl_Callback,
        arg3: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn Fl_Box_set_deleter(
        arg1: *mut Fl_Box,
        arg2: ::core::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
    );
}
extern "C" {
    pub fn Fl_Box_visible(self_: *const Fl_Box) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Box_visible_r(self_: *const Fl_Box) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Box_active(self_: *const Fl_Box) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Box_active_r(self_: *const Fl_Box) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Box_callback(self_: *const Fl_Box) -> Fl_Callback;
}
extern "C" {
    pub fn Fl_Box_set_deletion_callback(
        self_: *mut Fl_Box,
        arg1: ::core::option::Option<
            unsafe extern "C" fn(arg1: *mut Fl_Widget, arg2: *mut ::std::os::raw::c_void),
        >,
        data: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn Fl_Box_from_dyn_ptr(ptr: *mut Fl_Widget) -> *mut Fl_Box;
}
extern "C" {
    pub fn Fl_Box_super_draw(ptr: *mut Fl_Widget, flag: ::std::os::raw::c_int);
}
