use crate::utils;

struct MonthDayNode {
    pub month: u32,
    pub number_of_days: u32,
    next: Option<&'static MonthDayNode>
}

impl MonthDayNode {
    pub fn from(month: u32, year: u32) -> Self {
        MonthDayNode {
            month,
            number_of_days: utils::get_days_in_month(month, year),
            next: None
        }
    }

    pub fn append(&mut self, new_node: &'static MonthDayNode) {
        self.next = Some(new_node);
    }
}

pub struct MonthDaysCircularLinkedList<'a> {
    head: Option<&'a MonthDayNode>,
    tail: Option<&'a MonthDayNode>,
    pub length: u32,
}

impl MonthDaysCircularLinkedList<'static> {
    pub fn empty() -> Self {
        MonthDaysCircularLinkedList {
            head: None,
            tail: None,
            length: 0
        }
    }

    pub fn new(month: u32, year: u32) -> Self {
        let new_node = MonthDayNode::from(month, year);
        MonthDaysCircularLinkedList {
            head: Some(&new_node),
            tail: Some(&new_node),
            length: 1,
        }
    }

    pub fn complete(year: u32) -> Self {
        let mut circular_list = MonthDaysCircularLinkedList::empty();
        let mut capacity: u32 = 0;
        loop {
            if let None = circular_list.head {
                let new_node = MonthDayNode{
                    month: 1,
                    number_of_days: 31,
                    next: None,
                };
                circular_list.head = Some(&new_node);
                circular_list.tail = Some(&new_node);
                circular_list.length = 1;
                capacity += 1;
                continue;
            }
            let mut tail_value = circular_list.tail.unwrap();
            let new_node = MonthDayNode{
                month: capacity + 1,
                number_of_days: utils::get_days_in_month(capacity + 1, year),
                next: None,
            };
            tail_value.append(&new_node);
            circular_list.tail = Some(&new_node);
            circular_list.length = capacity + 1;
            capacity += 1;
            if capacity == 12 {
                break
            }
        }
        return circular_list;
    }

    fn append(&mut self, month: u32, year:u32) {
        let new_node = MonthDayNode::from(month, year);
        if self.length == 0 {
            self.head = Some(&new_node);
            self.tail = Some(&new_node);
            self.length = 1;
        } else {
            let last_node = self.head.unwrap_or(self.head.unwrap());
            self.tail.unwrap().append(last_node.clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::month_days::{MonthDayNode, MonthDaysCircularLinkedList};

    #[test]
    fn complete_creates_a_list_with_12_nodes() {
        let circular_list = MonthDaysCircularLinkedList::complete(2020);
        assert_eq!(circular_list.length, 12);
    }

    #[test]
    fn feb_has_29_days_in_leap_year() {
        let newNode = MonthDayNode::from(2, 1996);
        assert_eq!(newNode.number_of_days, 29);
    }

}