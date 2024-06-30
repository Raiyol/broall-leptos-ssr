use std::cmp::{max, min};

use leptos::*;
use thaw::{Button, ButtonSize, ButtonVariant};

use crate::beans::pageable::Pageable;

fn range(start: i64, end: i64) -> Vec<PaginationItem> {
    let mut ret = vec![];
    for idx in start..=end {
        ret.push(PaginationItem::Number(idx));
    }
    ret
}

enum PaginationItem {
    Dot,
    Number(i64),
}

fn use_pagination(page: i64, count: i64, sibling_count: i64) -> Vec<PaginationItem> {
    // Pages count is determined as siblingCount + firstPage + lastPage + currentPage + 2*DOTS
    let total_page_numbers = sibling_count + 5;
    // Case 1:
    //       If the number of pages is less than the page numbers we want to show in our
    //       paginationComponent, we return the range [1..totalPageCount]
    if total_page_numbers >= count {
        return range(1, count);
    }
    let current_page = page;
    // Calculate left and right sibling index and make sure they are within range 1 and totalPageCount
    let left_sibling_index = max(current_page - sibling_count, 1);
    let right_sibling_index = min(current_page + sibling_count, count);
    // We do not show dots just when there is just one page number to be inserted between the extremes of sibling and the page limits i.e 1 and totalPageCount.
    // Hence we are using leftSiblingIndex > 2 and rightSiblingIndex < totalPageCount - 2
    let should_show_left_dots = left_sibling_index > 2;
    let should_show_right_dots = right_sibling_index < count - 2;

    let first_page_index = 1;
    let last_page_index = count;

    // Case 2: No left dots to show, but rights dots to be shown
    if !should_show_left_dots && should_show_right_dots {
        let left_item_count = 3 + 2 * sibling_count;
        let mut left_range = range(1, left_item_count);
        left_range.push(PaginationItem::Dot);
        left_range.push(PaginationItem::Number(count));
        left_range
    } else if should_show_left_dots && !should_show_right_dots {
        // Case 3: No right dots to show, but left dots to be shown
        let right_item_count = 3 + 2 * sibling_count;
        let mut right_range = range(count - right_item_count + 1, count);
        let mut ret = vec![
            PaginationItem::Number(first_page_index),
            PaginationItem::Dot,
        ];
        ret.append(&mut right_range);
        ret
    } else {
        // Case 4: Both left and right dots to be shown
        let mut middle_range = range(left_sibling_index, right_sibling_index);
        let mut range = vec![
            PaginationItem::Number(first_page_index),
            PaginationItem::Dot,
        ];
        range.append(&mut middle_range);
        range.append(&mut vec![
            PaginationItem::Dot,
            PaginationItem::Number(last_page_index),
        ]);
        range
    }
}

#[component]
pub fn Pagination(
    #[prop(optional, into)] page: MaybeSignal<i64>,
    #[prop(into)] count: MaybeSignal<i64>,
    #[prop(optional, into)] sibling_count: MaybeSignal<i64>,
    #[prop(optional, into)] on_change: Option<Callback<i64>>,
) -> impl IntoView {
    let (page_signal, set_page_signal) = create_signal(page.get());
    let has_next = page.get() < count.get() - 1;
    let has_previous = page.get() > 0;
    logging::log!("{:?}", page);

    let on_click_previous = move |_| {
        set_page_signal.update(|val| *val -= 1);
        if let Some(callback) = on_change.as_ref() {
            callback.call(page_signal.get())
        }
    };

    let on_click_next = move |_| {
        set_page_signal.update(|val| *val -= 1);
        if let Some(callback) = on_change.as_ref() {
            callback.call(page_signal.get())
        }
    };

    view! {
        <nav class="pagination">
            <ul>
                <li>
                    <Button
                        on_click=on_click_previous
                        size=ButtonSize::Small variant=ButtonVariant::Text
                        icon=icondata::AiLeftOutlined disabled=!has_previous circle=true/>
                </li>
                {
                    move || use_pagination(page.get(), count.get(), sibling_count.get()).into_iter()
                        .map(|item| match item {
                        PaginationItem::Dot => view! {<li>"..."</li>},
                        PaginationItem::Number(nb) => view! {<li>
                            <Button
                                variant=MaybeSignal::derive(move || if page.get() == nb {ButtonVariant::Primary} else {ButtonVariant::Text})
                                on_click = move |_| {if page.get() != nb {set_page_signal.update(|val| *val = nb)}}
                                size=ButtonSize::Small circle=true>{nb}</Button>
                        </li>}
                    }).collect_view()
                }
                <li>
                    <Button
                        on_click=on_click_next
                        size=ButtonSize::Small variant=ButtonVariant::Text
                        icon=icondata::AiRightOutlined disabled=!has_next circle=true/>
                </li>
            </ul>
        </nav>
    }
}
