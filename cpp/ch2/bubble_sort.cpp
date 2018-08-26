#include <iostream>
using namespace std;

int bubbleSort(int A[], int N) {
  int sw = 0;
  bool flag = 1;
  for (int i = 0; flag; i++) {
    flag = 0;
    for (int j = N - 1; j >= i + 1; j--) {
      if (A[j] < A[j - i]) {
        swap(A[j], A[j - 1]);
        flag = 1;
        sw++;
      }
    }
  }
  return sw;
}
