#![allow(dead_code, unused_variables)]

// Singly Linked List Implementation

pub struct LinkedList {
	head: Link,
}

impl LinkedList {
	pub fn new() -> LinkedList {
		LinkedList { head: None }
	}

	pub fn insert_at_start(&mut self, item: u32) {
		let old_head = self.head.take();

		let new_head = Box::new(Node{
			data: item,
			next: old_head
		});

		self.head = Some(new_head)
	}

	pub fn pop(&mut self) {
		self.head = match self.head.take() {
			Some(x) => x.next,
			None => None,
		} 
	}

	pub fn print(&self) {
		print!("(Head) ");
		fn iterate(node: & Link) {
			match node {
				Some(x) => {
					print!("{} -> ", x.data);
					iterate(&x.next);
				},
				None => {
					println!("(None)");
					return
				},
			}
		}
		iterate(&self.head);
	}


}

struct Node {
	data: u32,
	next: Link,
}

type Link = Option<Box<Node>>;
