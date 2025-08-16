<?php

namespace App\Services\DataEditor\Fields;

interface FieldInterface
{
    public function getName(): string;
    public function getLabel(): string;
    public function getInputType(): string;
    public function getValue(): mixed;
    public function isEditable(): bool;
}
