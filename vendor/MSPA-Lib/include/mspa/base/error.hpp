#pragma once

#include <cstdint>
#include <string>

namespace mspa {

enum class ErrorCode : std::uint32_t {
    None = 0,
    InvalidArgument,
    UnknownAgent,
    DuplicateAgent,
    InvalidState,
    InvalidPack,
    QuotaExceeded,
    VersionMismatch,
    NamespaceConflict,
    TimeRegression,
    BatchConflict,
    OutOfMemory,
    UnsupportedOperation,
    SerializationError,
    InternalInvariantViolation
};

struct Error {
    ErrorCode code{ErrorCode::None};
    std::string details{};
};

} // namespace mspa
