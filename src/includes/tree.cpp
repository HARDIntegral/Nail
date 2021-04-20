#include "tree.h"

#define SUCCESS 1
#define FAILURE 0

template <class T>
Tree<T>::Tree(T data) {
  root->data = data;
  root->parent = nullptr;
  root->children = nullptr;
}

template <class T>
Tree<T>::~Tree() {
  // TODO: make destructor
}

template <class T>
int insert(tNode<T> *root, T data) {
  if (root == nullptr)
    return FAILURE;

  if (root->children == nullptr) {
    List<T> children_list;
    root->children = children_list;
  }
  (root->children).append(data);  
  return SUCCESS;
}

template <class T>
tNode<T> *generateBranch(tNode<T> *root, T data) {
  tNode<T> *newBranch = new tNode<T>;
  if (newBranch == nullptr)
    return nullptr;
  newBranch->data = data;
  newBranch->parent = root;
  newBranch->children = nullptr;
  return newBranch;
}

