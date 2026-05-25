#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

/// Error constants exposed to C.
constexpr static const int RMS_ERR_OK = 0;

constexpr static const int RMS_ERR_IO = 1;

constexpr static const int RMS_ERR_WOULD_BLOCK = 2;

constexpr static const int RMS_ERR_DISCONNECTED = 3;

constexpr static const int RMS_ERR_VERSION = 4;

constexpr static const int RMS_ERR_PANIC = 5;

/// Opaque connection handle.
struct RmsConn;

extern "C" {

/// Return the SS version string.
///
/// # Safety
/// Caller must later free the returned pointer with `rms_free_string`.
char *rms_version();

/// Return the last error code.
int rms_last_error();

/// Return the last error message.
///
/// # Safety
/// Caller must not free the returned pointer; it is thread-local and valid
/// until the next SS call on this thread.
const char *rms_last_error_message();

/// Connect to the RMS microkernel at the given socket path.
///
/// # Safety
/// `path` must be a valid NUL-terminated UTF-8 string.
RmsConn *rms_connect(const char *path);

/// Disconnect and free a connection.
///
/// # Safety
/// `conn` must be a valid pointer returned by `rms_connect`, or null.
void rms_disconnect(RmsConn *conn);

/// Return a pollable notification file descriptor.
///
/// # Safety
/// `conn` must be a valid pointer returned by `rms_connect`.
int rms_fd(const RmsConn *conn);

/// Free a string returned by SS.
///
/// # Safety
/// `s` must be a pointer returned by `rms_version` or null.
void rms_free_string(char *s);

}  // extern "C"
