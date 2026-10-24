#pragma once
#include <cstddef>
#include <map>
#include <string>
#include "mspa/base/result.hpp"
#include "mspa/decision/response_curve.hpp"
namespace mspa {struct ConsiderationDefinition{std::string id{};float weight{1};ResponseCurve curve{};};class ConsiderationRegistry{public:Result<void>add(ConsiderationDefinition);Result<void>remove(const std::string&);void clear()noexcept;[[nodiscard]]const ConsiderationDefinition*find(const std::string&)const noexcept;[[nodiscard]]std::size_t size()const noexcept;private:std::map<std::string,ConsiderationDefinition>definitions_{};};}
