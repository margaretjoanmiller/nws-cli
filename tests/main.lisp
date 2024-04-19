(defpackage nws-cli/tests/main
  (:use :cl
        :nws-cli
        :rove))
(in-package :nws-cli/tests/main)

;; NOTE: To run this test file, execute `(asdf:test-system :nws-cli)' in your Lisp.

(deftest test-target-1
  (testing "should (= 1 1) to be true"
    (ok (= 1 1))))
