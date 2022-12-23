<?php

class Point {
    public function __construct (public int $x, public int $y) {
    }
}

/**
 * @return Point[]
 */
function readPathFromString(string $line): array {
    return array_map(
        function (string $part) {
            [$x, $y] = explode(',', $part);
            return new Point($x, $y);
        },
        explode(' -> ', $line)
    );
}

function drawGrid(array $grid, int $minX, int $minY, $maxX, int $maxY) {
    $width = count($grid);
    $height = count($grid[0]);

    for ($y = $minY; $y < $height; $y++) {
        for ($x = $minX; $x < $width; $x++) {
            if ($grid[$x][$y]) {
                echo '#';
            } else {
                echo '.';
            }
        }
        echo PHP_EOL;
    }
}

$input = file_get_contents('assets/puzzle.input');
/** @var Point[][] $paths */
$paths = array_map(
    fn (string $line) => readPathFromString($line),
    explode(PHP_EOL, $input)
);

$flowFromX = 500;
$flowFromY = 0;

$minX = $flowFromX;
$minY = $flowFromY;
$maxX = $flowFromX;
$maxY = $flowFromY;
foreach ($paths as $path) {
    foreach ($path as $point) {
        $minX = min($minX, $point->x);
        $minY = min($minY, $point->y);        
        $maxX = max($maxX, $point->x);
        $maxY = max($maxY, $point->y);
    }
}
$maxX += 1;
$maxY += 1;
// TODO We shoudl also expand minX and minY, if minX and minY are alreaydy 0
// $minX -= 1;
// $minY -= 1;

$grid = [];
for ($x = 0; $x <= $maxX; $x++) {
    $grid[$x] = [];
    for ($y = 0; $y <= $maxY; $y++) {
        $grid[$x][$y] = false;
    }
}

foreach ($paths as $path) {
    for ($i = 0; $i < count($path) - 1; $i++) {
        $start = $path[$i];
        $end = $path[$i + 1];

        if ($start->x === $end->x) {
            for ($y = min($start->y, $end->y); $y <= max($start->y, $end->y); $y++) {
                $grid[$start->x][$y] = true;
            }
        } else if ($start->y === $end->y) {
            for ($x = min($start->x, $end->x); $x <= max($start->x, $end->x); $x++) {
                $grid[$x][$start->y] = true;
            }
        }
    }
}

function insertSand(array &$grid, int $fromX, int $fromY): bool {
    $width = count($grid);
    $height = count($grid[0]);

    $x = $fromX;
    $y = $fromY;
    do {
        if ($x < 0 || $x >= $width || $y < 0 || $y >= $height) {
            return true;
        }

        // Invariant: (x, y) is free

        if (!($grid[$x][$y + 1] ?? false)) {
            $y = $y + 1;
        } else if ($grid[$x][$y + 1] ?? false) {
            if (!($grid[$x - 1][$y + 1] ?? false)) {
                $x -= 1;
                $y += 1;
            } else if (!($grid[$x + 1][$y + 1] ?? false)) {
                $x += 1;
                $y += 1;
            } else {
                $grid[$x][$y] = true;
                break;
            }
        }

    } while (true);

    return false;
}

drawGrid($grid, $minX, $minY, $maxX, $maxY);

$dropped = 0;
do {
    $overflow = insertSand($grid, $flowFromX, $flowFromY);
    if ($overflow) {
        break;
    }

    $dropped++;

} while(true);

drawGrid($grid, $minX, $minY, $maxX, $maxY);   

echo $dropped;