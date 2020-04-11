<?php
// require_once 'Figure.php';


// class Triangle extends Figure {
//     public function getArea(): float {
//         return $this->width * $this->height / 2;
//     }
// }

require_once 'IFigure.php';


class Triangle implements IFigure {
    private $width;
    private $height;

    public function __construct(float $width, float $height) {
        $this->width = $width;
        $this->height = $height;
    }

    public function getArea(): float {
        return $this->width * $this->height / 2;
    }
}