(ns day5.core
  (:require [clojure.string :as str])
  (:gen-class))

(defn load-data []
  (let [file (slurp "input")
        lines (str/split-lines file)]
	(vec (map #(Integer/parseInt %) lines))))

(defn solve-maze [input]
  (let [maze-length (count input)]
    (loop [maze input
           pos 0
           steps 0]
      (if (>= pos maze-length)
        steps
	(let
	  [jump (nth maze pos)
	   new-maze (assoc maze pos (inc jump))
	   new-pos (+ pos jump)
	   new-step (inc steps)]
	  (recur new-maze new-pos new-step))))))

(defn new-maze-2 [maze pos]
  (let [jump (nth maze pos)]
    (if (< 2 jump)
      (assoc maze pos (dec jump))
      (assoc maze pos (inc jump)))))

(defn solve-maze-2 [input]
  (let [maze-length (count input)]
    (loop [maze input
           pos 0
	   steps 0]
      (if (>= pos maze-length)
        steps
	(let
	  [jump (nth maze pos)
	   new-maze (new-maze-2 maze pos)
	   new-pos (+ pos jump)
	   new-step (inc steps)]
	  (recur new-maze new-pos new-step))))))

(defn -main []
  (let [input     (load-data)
        solution1 (solve-maze input)
	solution2 (solve-maze-2 input)]
	(println (str "Solution 1: " solution1))
	(println (str "Solution 2: " solution2))))