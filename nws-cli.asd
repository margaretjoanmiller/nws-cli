(defsystem "nws-cli"
           :long-name "National Weather Service Command Line Interface"
           :version "0.0.1"
           :author "Margaret Miller"
           :maintainer "Margaret Miller"
           :mailto "maeborow@posteo.net"
           :license "LGPL"
           :homepage "https://github.com/fukamachi/cl-project"
           :bug-tracker "https://github.com/fukamachi/cl-project/issues"
           :source-control "https://github.com/fukamachi/cl-project.git"
           :depends-on ("dexador"
                        "com.inuoe.jzon"
                        "binding-arrows"
                        "adopt")
           :components ((:module "src"
                                 :components
                                 ((:file "main")
                                  (:file "util"))))
           :description "NWS CLI"
           :long-description "Command line interface for the National Weather Service"
           :in-order-to ((test-op (test-op "nws-cli/tests"))))

(defsystem "nws-cli/tests"
  :author "Margaret Miller"
  :license "LGPL"
  :depends-on ("nws-cli"
               "rove")
  :components ((:module "tests"
                :components
                ((:file "main"))))
  :description "Test system for nws-cli"
  :perform (test-op (op c) (symbol-call :rove :run c)))
