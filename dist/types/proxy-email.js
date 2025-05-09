"use strict";
/**
 * Type definitions for proxy email models
 */
Object.defineProperty(exports, "__esModule", { value: true });
exports.SpamFilterLevel = exports.ProxyEmailStatus = void 0;
var ProxyEmailStatus;
(function (ProxyEmailStatus) {
    ProxyEmailStatus["Active"] = "Active";
    ProxyEmailStatus["Disabled"] = "Disabled";
    ProxyEmailStatus["Deleted"] = "Deleted";
})(ProxyEmailStatus || (exports.ProxyEmailStatus = ProxyEmailStatus = {}));
var SpamFilterLevel;
(function (SpamFilterLevel) {
    SpamFilterLevel["Low"] = "Low";
    SpamFilterLevel["Medium"] = "Medium";
    SpamFilterLevel["High"] = "High";
    SpamFilterLevel["VeryHigh"] = "VeryHigh";
})(SpamFilterLevel || (exports.SpamFilterLevel = SpamFilterLevel = {}));
//# sourceMappingURL=proxy-email.js.map