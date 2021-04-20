#pragma once

#include <iostream>

#include "list.h" 

template <class T>
struct tNode {
  T data;
  tNode<T> *parent;
  node<T> *children;
};

template <class T>
class Tree {
  public:
    Tree(T data);
    ~Tree();
    int insert(tNode<T> *root, T data);
    int retrieveData(tNode<T> *root);
    int retrieveChildren(tNode<T> *root);
    int burnBranch(tNode<T> *root);
  private:
    tNode<T> *root;

    tNode<T> *generateBranch(tNode<T> *root, T data);
};
