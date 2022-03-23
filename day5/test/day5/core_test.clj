(ns day5.core-test
  (:require [clojure.test :refer :all]
            [day5.core :refer :all]))

(deftest solution-1
  (testing "Solution 1"
    (let [input (load-data)
          solution1 (solve-maze input)]
      (is (= solution1 315613)))))

(deftest solution-2
  (testing "Solution 2"
    (let [input (load-data)
          solution2 (solve-maze-2 input)]
      (is (= solution2 22570529)))))
