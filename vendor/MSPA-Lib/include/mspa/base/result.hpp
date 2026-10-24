#pragma once
#include <string>
#include <utility>
#include <variant>
#include "mspa/base/error.hpp"
namespace mspa {
template <typename T> class Result {
public:
    Result(const T& v):storage_(v){} Result(T&& v):storage_(std::move(v)){}
    Result(const Error& e):storage_(e){} Result(Error&& e):storage_(std::move(e)){}
    [[nodiscard]] bool has_value() const noexcept { return std::holds_alternative<T>(storage_); }
    [[nodiscard]] explicit operator bool() const noexcept { return has_value(); }
    [[nodiscard]] T* value_if() noexcept { return std::get_if<T>(&storage_); }
    [[nodiscard]] const T* value_if() const noexcept { return std::get_if<T>(&storage_); }
    [[nodiscard]] Error* error_if() noexcept { return std::get_if<Error>(&storage_); }
    [[nodiscard]] const Error* error_if() const noexcept { return std::get_if<Error>(&storage_); }
private: std::variant<T,Error> storage_;
};
template <> class Result<void> {
public:
    Result()=default; Result(const Error& e):error_(e),ok_(false){} Result(Error&& e):error_(std::move(e)),ok_(false){}
    [[nodiscard]] bool has_value() const noexcept { return ok_; } [[nodiscard]] explicit operator bool() const noexcept { return ok_; }
    [[nodiscard]] const Error* error_if() const noexcept { return ok_?nullptr:&error_; }
private: Error error_{}; bool ok_{true};
};
inline Error make_error(ErrorCode c, std::string d={}) { return Error{c,std::move(d)}; }
} // namespace mspa
