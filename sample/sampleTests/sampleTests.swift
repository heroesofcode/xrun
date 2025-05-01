//
//  sampleTests.swift
//  sampleTests
//
//  Created by João Lucas on 01/05/25.
//

import XCTest
@testable import sample

final class sampleTests: XCTestCase {

    func testSum() {
        XCTAssertEqual(5, 2+3)
    }
    
    func testMyName() {
        XCTAssertEqual("João Lucas", "João Lucas")
    }
}
