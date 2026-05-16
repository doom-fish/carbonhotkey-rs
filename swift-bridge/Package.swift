// swift-tools-version:5.9
import PackageDescription

let package = Package(
    name: "CarbonHotKeyBridge",
    platforms: [
        .macOS(.v10_13)
    ],
    products: [
        .library(
            name: "CarbonHotKeyBridge",
            type: .static,
            targets: ["CarbonHotKeyBridge"])
    ],
    targets: [
        .target(
            name: "CarbonHotKeyBridge",
            path: "Sources/CarbonHotKeyBridge",
            publicHeadersPath: "include")
    ]
)
