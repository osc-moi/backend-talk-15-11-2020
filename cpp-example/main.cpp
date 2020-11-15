#include <thread>
#include <string>
#include <iostream>
#include <vector>
#include <unistd.h>

auto second = 1000000; // 1 second

void print(int n, const std::string &str)  {
  usleep(second);
  std::string msg = std::to_string(n) + " : " + str;
  std::cout << msg  << std::endl;
}
 
int main() {
  std::vector<std::string> s(100,"person done eating") ;
  std::vector<std::thread> threads;
 
  for (uint i = 0; i < s.size(); i++) {
    threads.push_back(std::thread(print, i, s[i]));
  }
 
  for (auto &th : threads) {
    th.join();
  }
  return 0;
}

// g++ -pthread -o cpp cpp-examples/main.cpp && ./cpp