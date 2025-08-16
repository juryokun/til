<?php
/*
 * trait学習
 */

// interface 定義
interface IFax {
    function send();
}
interface IPrinter {
    function print();
}


// trait 定義
trait FaxTrait {
    public function send() {
        print 'sending Fax ... sended!';
    }
}
trait PrinterTrait {
    public function print() {
        print 'printing ... completed!';
    }
}


// 複数のinterfaceの実装をtraitで定義
class FaxPrinter implements IFax, IPrinter {
    use FaxTrait, PrinterTrait;
}

$fp = new FaxPrinter();
$fp->send();
$fp->print();
