/**
 * Some glue definitions for rust bindgen
 */

#include "sqlite3.h"

/**
 * @brief See SQLITE_TRANSIENT
 * 
 * @note Necessary due to limitations in bindgen and Rust's strict pointer casting rules
 */
sqlite3_destructor_type sqlite3_transient() {
    return SQLITE_TRANSIENT;
}
