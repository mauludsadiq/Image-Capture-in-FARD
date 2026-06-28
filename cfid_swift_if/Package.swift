// swift-tools-version:5.5
import PackageDescription

let package = Package(
    name: "IFReceipt",
    platforms: [.macOS(.v12)],
    products: [
        .library(name: "IFReceipt", targets: ["IFReceipt"]),
    ],
    targets: [
        .target(name: "IFReceipt", dependencies: []),
        .testTarget(name: "IFReceiptTests", dependencies: ["IFReceipt"]),
    ]
)
