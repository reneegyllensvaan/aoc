; Hailstone: 19, 13, 30 @ -2, 1, -2
; Collision time: 5
; Collision position: 9, 18, 20
;
; Hailstone: 18, 19, 22 @ -1, -1, -2
; Collision time: 3
; Collision position: 15, 16, 16
;
; Hailstone: 20, 25, 34 @ -2, -2, -4
; Collision time: 4
; Collision position: 12, 17, 18
;
; Hailstone: 12, 31, 28 @ -1, -2, -1
; Collision time: 6
; Collision position: 6, 19, 22
;
; Hailstone: 20, 19, 15 @ 1, -5, -3
; Collision time: 1
; Collision position: 21, 14, 12


(echo "hi there")
(declare-const x0 Int)
(declare-const y0 Int)
(declare-const z0 Int)

(declare-const dx Int)
(declare-const dy Int)
(declare-const dz Int)

(declare-const t1 Int)
(declare-const t2 Int)
(declare-const t3 Int)

; first hailstone
(assert (= (+ x0 (* dx t1)) (+ 237822270988608 (* 115 t1))))
(assert (= (+ y0 (* dy t1)) (+ 164539183264530 (* 346 t1))))
(assert (= (+ z0 (* dz t1)) (+ 381578606559948 (* -342 t1))))
; second hailstone
(assert (= (+ x0 (* dx t2)) (+ 287838354624648 (* -5 t2))))
(assert (= (+ y0 (* dy t2)) (+ 284335343503076 (* -84 t2))))
(assert (= (+ z0 (* dz t2)) (+ 181128681512377 (* 175 t2))))
; third hailstone
(assert (= (+ x0 (* dx t3)) (+ 341046208911993 (* -74 t3))))
(assert (= (+ y0 (* dy t3)) (+ 120694764237967 (* 129 t3))))
(assert (= (+ z0 (* dz t3)) (+ 376069872241870 (* -78 t3))))

; ; first hailstone
; (assert (= (+ x0 (* dx t1)) (+ 19 (* -2 t1))))
; (assert (= (+ y0 (* dy t1)) (+ 13 (* 1 t1))))
; (assert (= (+ z0 (* dz t1)) (+ 30 (* -2 t1))))
;
; ; second hailstone
; (assert (= (+ x0 (* dx t2)) (+ 18 (* -1 t2))))
; (assert (= (+ y0 (* dy t2)) (+ 19 (* -1 t2))))
; (assert (= (+ z0 (* dz t2)) (+ 22 (* -2 t2))))
;
; ; third hailstone
; (assert (= (+ x0 (* dx t3)) (+ 20 (* -2 t3))))
; (assert (= (+ y0 (* dy t3)) (+ 25 (* -2 t3))))
; (assert (= (+ z0 (* dz t3)) (+ 34 (* -4 t3))))

(check-sat)
(get-model)
