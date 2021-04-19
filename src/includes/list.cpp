#include <cmath>

#include "list.h"

#define SUCCESS 1
#define FAILURE 0

// Constructor to generate list
template <class T>
List<T>::List() {
  head = nullptr;
  tail = nullptr;
  size = 0;
}

// Destructor to free memory
template <class T>
List<T>::~List() {
  free(head);
  free(tail);
}

// Adds values to the head of the list
template <class T>
int List<T>::push(T data) {
  node<T> *newNode = generateNode(data);

  if (newNode == nullptr)
    return FAILURE;
  if (size == 0) 
    tail = newNode;
  else {
    head->prev = newNode;
    newNode->next = head;
  }
  head = newNode;
  size++;
  return SUCCESS;
}

// Adds values to the tail of the list
template <class T>
int List<T>::append(T data) {
  node<T> *newNode = generateNode(data);

  if (newNode == nullptr)
    return FAILURE;
  if (size == 0)
    head = newNode;
  else {
    tail->next = newNode;
    newNode->prev = tail;
  }
  tail = newNode;
  size++;
  return SUCCESS;
}

// Retrieves data from the head of the list
template <class T>
T *List<T>::getHead() {
  return &(head->data);
}

// Retrieves data from the tail of the list
template <class T>
T *List<T>::getTail() {
  return &(tail->data);
}

// Retrieves data from a given index in a list
template <class T>
T *List<T>::retriveData(int position){
  node<T> *currentNode = retriveNode(position);
  
  if (currentNode == nullptr)
    return nullptr;
  else 
    return &(currentNode->data);
}

// Returns the value of the head of the list and then destroys it
template <class T>
T *List<T>::pop() {
  node<T> *currentNode = head;
  if (currentNode == nullptr)
    return nullptr;
  
  T *data = removeNode(0);
  if (data == nullptr)
    return nullptr;
  return data;
}

// Returns the value of the tail of the list and then destroys it
template <class T>
T *List<T>::snip() {
  node<T> *currentNode = tail;
  if (currentNode == nullptr)
    return nullptr;
  
  T* data = removeNode(size - 1);
  if (data == nullptr)
    return nullptr;
  return data;
}

// Returns the valuse of a node of a given position then destroys it
template <class T>
T *List<T>::removeNode(int position) {
  node<T> *currentNode = retriveNode(position);
  T *data = new T;
  T *data_holder;

  if (currentNode == nullptr)
    return nullptr;

  if (currentNode->next == nullptr)
    tail = currentNode->prev;
  else
    currentNode->next->prev = currentNode->prev;

  if (currentNode->prev == nullptr)
    head = currentNode->next;
  else
    currentNode->prev->next = currentNode->next;

  data_holder = &(currentNode->data);
  *data = *data_holder;
  delete currentNode;
  return data;
}

// Returns the length of the list
template <class T>
int List<T>::length() {
  return size;
}

// Iterates through a list with a given function
template <class T>
void List<T>::iterate(void (*fn)(T), int reverse) {
  node<T> *currentNode = (reverse ? tail : head);

  while (currentNode != nullptr) {
    (*fn)(currentNode->data);
    currentNode = (reverse ? (currentNode->prev) : (currentNode->next));
  }
}

// Creates a new node for the list
template <class T>
node<T> *List<T>::generateNode(T data) {
  node<T> *newNode = new node<T>;
  if (newNode == nullptr)
    return nullptr;
  newNode->data = data;
  newNode->next = nullptr;
  newNode->prev = nullptr;
  return newNode;
}

// Retrieves a node in the list given an index
template <class T>
node<T> *List<T>::retriveNode(int position) {
  if (position > size - 1)
    return nullptr;
  
  node<T> *currentNode;
  int currentPosition;
  int reverse;

  if (position > (size -1) / 2) {
    currentNode = tail;
    currentPosition = size - 1;
    reverse = 1;
  } else {
    currentNode = head;
    currentPosition = 0;
    reverse = 0;
  }

  while (currentNode != nullptr) {
    if (currentPosition == position) 
      break;
    
    currentNode = (reverse ? (currentNode->prev) : (currentNode->next));
    currentPosition = (reverse ? (currentPosition - 1) : (currentPosition + 1));
  }

  return currentNode;
}

template class List<int>;
