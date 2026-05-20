use format_num::format_num;

pub fn format_num<T: Into<usize>>(item: T) -> String {
    let val = item.into();
    if val > 10000 {
        format_num!(".3e", val as f64)
    } else {
        format_num!(".0f", val as f64)
    }
}
