"use strict";
/**
 * Type definitions for HIPAA compliance models
 */
Object.defineProperty(exports, "__esModule", { value: true });
exports.AccessType = exports.UserRole = void 0;
var UserRole;
(function (UserRole) {
    UserRole["Patient"] = "Patient";
    UserRole["Doctor"] = "Doctor";
    UserRole["Nurse"] = "Nurse";
    UserRole["Admin"] = "Admin";
    UserRole["Technician"] = "Technician";
    UserRole["Auditor"] = "Auditor";
})(UserRole || (exports.UserRole = UserRole = {}));
var AccessType;
(function (AccessType) {
    AccessType["View"] = "View";
    AccessType["Create"] = "Create";
    AccessType["Update"] = "Update";
    AccessType["Delete"] = "Delete";
    AccessType["Export"] = "Export";
    AccessType["Import"] = "Import";
    AccessType["Share"] = "Share";
    AccessType["EmergencyAccess"] = "EmergencyAccess";
})(AccessType || (exports.AccessType = AccessType = {}));
//# sourceMappingURL=hipaa-compliance.js.map