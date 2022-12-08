#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn is_in<T: PartialEq>(parent_list: &[T], child_list: &[T], child_list_len: usize) -> bool {
    parent_list
        .windows(child_list_len)
        .any(|sublist| child_list == sublist)
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    let len1st = _first_list.len();
    let len2nd = _second_list.len();

    if len1st == len2nd && (_first_list.is_empty() || is_in(_first_list, _second_list, len2nd)) {
        Comparison::Equal
    } else if len1st > len2nd
        && (_second_list.is_empty() || is_in(_first_list, _second_list, len2nd))
    {
        Comparison::Superlist
    } else if len1st < len2nd
        && (_first_list.is_empty() || is_in(_second_list, _first_list, len1st))
    {
        Comparison::Sublist
    } else {
        Comparison::Unequal
    }
}
