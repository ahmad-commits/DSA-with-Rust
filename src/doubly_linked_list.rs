// Doubly Linked List Implementation

#![allow(dead_code)]
use std::ptr::null_mut;

type NodePtr = *mut Node;

struct Node {
	data: u32,
	prev: NodePtr,
	next: NodePtr,
}

pub struct DoublyLinkedList {
	head: NodePtr,
	tail: NodePtr
}

impl DoublyLinkedList {
	pub fn new() -> DoublyLinkedList {
		DoublyLinkedList { head: null_mut(), tail: null_mut() }
	}

	pub fn insert_at_start(&mut self, item: u32) {
		let node = &mut Node {data: item, prev: null_mut(), next: null_mut()} as NodePtr;
		if self.tail.is_null() {
			self.head = node;
			self.tail = node;
		} else {
			unsafe {
				(*self.tail).next = node;
				(*node).prev = self.tail;
				self.tail = node 
			}
		}
	}

	pub fn print(&self) {
		let mut current_node = self.head;
		unsafe {
		while !current_node.is_null() {
			current_node = (*current_node).next;
		}
	}
	}

}