<?php

namespace App\Services\DataEditor\Interface;

interface EditableTableServiceInterface
{
    public function getFields(): array;
    public function getData(): array;
    public function updateData(array $data): void;
}
