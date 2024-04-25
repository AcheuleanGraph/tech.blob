//! Rust's circular linked list
//! 
//! NodeA:
//!   * use Rc<RefCell<>> as pointer
//! NodeB:
//!   * use raw pointer


use std::rc::Rc;
use std::cell::RefCell;
use std::ptr;


/// use Rc<RefCell<>> as pointer
pub struct NodeA<T: Clone>{
  pub data: T,
  pub prev: Option<Rc<RefCell<NodeA<T>>>>,
  pub i: usize,
  pub next: Option<Rc<RefCell<NodeA<T>>>>
}

impl<T: Clone> NodeA<T> {

  fn _link(prev: Rc<RefCell<Self>>, mut data_list: Vec<T>, i: usize) -> Rc<RefCell<Self>> {

    let data = data_list.remove(0);
    let node = Self { data, prev: Some(prev.clone()), i, next: None };
    let node = Rc::new(RefCell::new(node));
    prev.borrow_mut().next = Some(node.clone());

    if data_list.len()>0 {
      Self::_link(node, data_list, i+1)
    } else {
      node
    }
  }

  pub fn init(mut data_list: Vec<T>) -> Option<Rc<RefCell<Self>>> {

    if data_list.is_empty() {
      return None;
    }

    let data = data_list.remove(0);
    let head = Self { data, prev: None, i: 0, next: None };
    let head = Rc::new(RefCell::new(head));

    if data_list.len()>0 {
      let tail = Self::_link(head.clone(), data_list, 1);
      head.borrow_mut().prev = Some(tail.clone());
      tail.borrow_mut().next = Some(head.clone());
    } else {
      head.borrow_mut().prev = Some(head.clone());
      head.borrow_mut().next = Some(head.clone());
    }

    Some(head)
  }

  pub fn tour(node: &Rc<RefCell<Self>>) -> Vec<T> {

    let mut data_list = vec![];
    let i = node.as_ref().borrow().i;
    let mut node = node.clone();
    
    loop {
      data_list.push(node.as_ref().borrow().data.clone());
      let next = node.as_ref().borrow().next.clone().unwrap();
      if next.as_ref().borrow().i==i {
        break;
      }
      node = next;
    }

    data_list
  }

  pub fn len(node: &Rc<RefCell<Self>>) -> usize {
    let i = node.as_ref().borrow().i;
    let mut node = node.clone();
    let mut len = 0;

    loop {
      len += 1;
      let next = node.as_ref().borrow().next.clone().unwrap();
      if next.as_ref().borrow().i==i {
        break;
      }
      node = next;
    }

    len
  }

  pub fn new_id(node: &Rc<RefCell<Self>>) -> usize {

    let mut id_list = vec![];
    let i = node.as_ref().borrow().i;
    let mut node = node.clone();
    id_list.push(i);

    loop {
      let next = node.as_ref().borrow().next.clone().unwrap();
      let next_i = next.as_ref().borrow().i;
      if next_i==i {
        break;
      }
      id_list.push(next_i);
      node = next;
    }

    (0..id_list.len()+1).find(|x| !id_list.contains(x)).unwrap()
  }

  pub fn step(node: &Rc<RefCell<Self>>, step: usize) -> Rc<RefCell<Self>> {

    let mut node = node.clone();
    for _ in 0..step {
      let next = node.as_ref().borrow().next.clone().unwrap();
      node = next;
    }
    node
  }

  pub fn cut(node: &mut Rc<RefCell<Self>>) -> Option<(T, Rc<RefCell<Self>>)> {
    
    let node_ = node.as_ref().borrow();
    let prev = node_.prev.clone().unwrap();
    let next = node_.next.clone().unwrap();
    let data = node_.data.clone();

    prev.borrow_mut().next.replace(next.clone());
    next.borrow_mut().prev.replace(prev);

    Some((data, next))
  }

  pub fn add(prev: &mut Rc<RefCell<Self>>, data: T, new_id: Option<usize>) -> Rc<RefCell<Self>> {

    let new_id = new_id.unwrap_or(Self::new_id(prev));
    let prev_next = prev.as_ref().borrow().next.clone().unwrap();
    let next = Self {
      data, i: new_id, prev: Some(prev.clone()), next: Some(prev_next.clone())
    };
    let next = Rc::new(RefCell::new(next));

    prev.borrow_mut().next.replace(next.clone());
    prev_next.borrow_mut().prev.replace(next.clone());

    next
  }
}



/// use raw pointer
pub struct NodeB<T: Clone>{
  pub data: T,
  pub prev: *mut NodeB<T>,
  pub i: usize,
  pub next: *mut NodeB<T>
}


impl<T: Clone> NodeB<T> {

  unsafe fn _link(prev: *mut Self, mut data_list: Vec<T>, i: usize) -> *mut Self {
    let data = data_list.remove(0);
    let node = Self { data, prev, i, next: ptr::null_mut() };
    let node = Box::into_raw(Box::new(node));
    (*prev).next = node;

    if data_list.len()>0 {
      Self::_link(node, data_list, i+1)
    } else {
      node
    }
  }

  pub fn init(mut data_list: Vec<T>) -> Option<*mut Self> {
    unsafe {
      if data_list.is_empty() {
        return None;
      }

      let data = data_list.remove(0);
      let head = Self { data, prev: ptr::null_mut(), i: 0, next: ptr::null_mut() };
      let head = Box::into_raw(Box::new(head));

      if data_list.len()>0 {
        let tail = Self::_link(head, data_list, 1);
        (*head).prev = tail;
        (*tail).next = head;
      } else {
        (*head).prev = head;
        (*head).next = head;
      }

      Some(head)
    }
  }

  pub fn tour(mut node: *mut Self) -> Vec<T> {
    unsafe {
      let mut data_list = vec![];
      let i = (*node).i;
      
      loop {
        data_list.push((*node).data.clone());
        let next = (*node).next;
        if (*next).i==i {
          break;
        }
        node = next;
      }

      data_list
    }
  }

  pub fn len(mut node: *mut Self) -> usize {
    unsafe {
      let i = (*node).i;
      let mut len = 0;
  
      loop {
        len += 1;
        let next = (*node).next;
        if (*next).i==i {
          break;
        }
        node = next;
      }
  
      len
    }
  }

  pub fn new_id(node: *mut Self) -> usize {
    unsafe {
      let mut id_list = vec![];
      let i = (*node).i;
      let mut node = node.clone();
      id_list.push(i);

      loop {
        let next = (*node).next;
        let next_i = (*next).i;
        if next_i==i {
          break;
        }
        id_list.push(next_i);
        node = next;
      }

      (0..id_list.len()+1).find(|x| !id_list.contains(x)).unwrap()
    }
  }

  pub fn cut(node: *mut Self) -> Option<(T, *mut Self)> {
    unsafe {          
      let prev = (*node).prev;
      let next = (*node).next;
      let data = (*node).data.clone();

      // destruct cut out node
      let _ = Box::from_raw(node);

      (*prev).next = next;
      (*next).prev = prev;

      Some((data, next))
    }
  }

  pub fn step(mut node: *mut Self, step: usize) -> *mut Self {
    unsafe {
      for _ in 0..step {
        let next = (*node).next;
        node = next;
      }
      node
    }
  }

  pub fn add(prev: *mut Self, data: T, new_id: Option<usize>) -> *mut Self {
    unsafe {
      let new_id = new_id.unwrap_or(Self::new_id(prev));
      let prev_next = (*prev).next;
      let next = Self {
        data, i: new_id, prev, next: prev_next
      };
      let next = Box::into_raw(Box::new(next));

      (*prev).next = next;
      (*prev_next).prev = next;

      next
    }
  }

  pub fn clean(mut node: *mut Self) {
    unsafe {
      let prev = (*(*node).prev).i;
      loop {
        let i = (*node).i;
        if i==prev {
          let _ = Box::from_raw(node);
          break
        } else {
          let next = (*node).next;
          let _ = Box::from_raw(node);
          node = next;
        }
      }
    }
  }
}



#[cfg(test)]
mod tests {
  use super::*;

  fn data_() -> Vec<i32> {
    (0..20).collect()
  }

  #[test]
  #[cfg_attr(miri, ignore)]
  fn test_node_a() {

    let mut node = NodeA::init(data_()).unwrap();

    let mut node = NodeA::step(&mut node, 10);
    assert_eq!(node.as_ref().borrow().data, 10);

    let (data, mut node) = NodeA::cut(&mut node).unwrap();
    assert_eq!(data, 10);
    assert_eq!(node.as_ref().borrow().data, 11);

    let node = NodeA::add(&mut node, 0, None);
    let tour = NodeA::tour(&node);
    let tour_ = vec![
      vec![0], (12..20).collect::<Vec<_>>(),
      (0..10).collect::<Vec<_>>(), vec![11]
    ].concat();
    assert_eq!(tour, tour_);
  }

  #[test]
  fn test_node_b() {
    unsafe {
      let node = NodeB::init(data_()).unwrap();

      let node = NodeB::step(node, 10);
      assert_eq!((*node).data, 10);

      let (data, node) = NodeB::cut(node).unwrap();
      assert_eq!(data, 10);
      assert_eq!((*node).data, 11);

      let node = NodeB::add(node, 0, None);
      let tour = NodeB::tour(node);
      let tour_ = vec![
        vec![0], (12..20).collect::<Vec<_>>(),
        (0..10).collect::<Vec<_>>(), vec![11]
      ].concat();
      assert_eq!(tour, tour_);

      NodeB::clean(node);
    }
  }
}