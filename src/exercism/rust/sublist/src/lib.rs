#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn check_sublist<T: PartialEq + std::fmt::Debug>(long_list: &[T], short_list: &[T]) -> Comparison {
    for i in 0..(long_list.len() - short_list.len() + 1) {
        let mut found = true;
        for j in 0..short_list.len() {
            if long_list[i + j] != short_list[j] {
                found = false;
            }
        }

        if found == true {
            return Comparison::Superlist;
        }
    }

    Comparison::Unequal
}

pub fn sublist<T: PartialEq + std::fmt::Debug>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    let len_1st = _first_list.len();
    let len_2nd = _second_list.len();

    // First off, check if they are equal
    if _first_list.eq(_second_list) {
        return Comparison::Equal;
    }

    if len_1st == 0 {
        return Comparison::Sublist;
    }

    if len_2nd == 0 {
        return Comparison::Superlist;
    }

    let mut is_first_list_longer = true;

    let mut ret = if len_1st >= len_2nd {
        check_sublist(_first_list, _second_list)
    } else {
        is_first_list_longer = false;
        check_sublist(_second_list, _first_list)
    };

    if is_first_list_longer == false {
        if ret == Comparison::Superlist {
            ret = Comparison::Sublist;
        } else if ret == Comparison::Sublist {
            ret = Comparison::Superlist;
        }
    }

    return ret;
}
