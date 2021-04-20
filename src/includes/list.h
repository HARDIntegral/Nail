#pragma once

#include <iostream>

template <class T>
struct node {
  T data;
  node<T> *next;
  node<T> *prev;
};

template <class T>
class List {
  public:
    // Generate a list
    List();
    // Destroys list
    ~List();
    // Add values to the list
    int push(T data);
    int append(T data);
    // Non-destructive getters
    T *getHead();
    T *getTail();
    T *retrieveData(int position);
    // Destructive getters
    T *pop();
    T *snip();
    T *removeNode(int position);
    // Other functions
    int length();
    void iterate(void (*fn)(T), int reverse);
  private:
    node<T> *head;
    node<T> *tail;
    int size;

    node<T> *generateNode(T data); 
    node<T> *retrieveNode(int position); 
};
