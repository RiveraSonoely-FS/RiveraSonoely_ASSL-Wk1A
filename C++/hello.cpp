// main.cpp
#include <iostream>
#include <chrono>

int main() {

    auto current_time = std::chrono::system_clock::now();

    std::time_t now_c = std::chrono::system_clock::to_time_t(current_time);

    std::string current_time_str = std::ctime(&now_c);

    std::cout << "Hello World! " << current_time_str;
    return 0;
}
