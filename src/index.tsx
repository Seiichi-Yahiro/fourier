import { chunk } from 'lodash';
import { RotatingCircles } from '../wasm_build';

const generateDrawFunction = (rotatingCircles: RotatingCircles) => {
    const linesCanvas = document.getElementById('canvas')! as HTMLCanvasElement;
    const linesContext = linesCanvas.getContext('2d')!;
    const pointsCanvas = document.createElement('canvas') as HTMLCanvasElement;
    const pointsContext = pointsCanvas.getContext('2d')!;
    pointsCanvas.width = linesCanvas.width;
    pointsCanvas.height = linesCanvas.height;
    const halfWidth = linesCanvas.width / 2;
    const halfHeight = linesCanvas.height / 2;
    pointsContext.translate(halfWidth, halfHeight);

    return (t: number) => {
        linesContext.clearRect(-halfWidth, -halfHeight, linesCanvas.width, linesCanvas.height);
        linesContext.beginPath();

        const points = chunk(rotatingCircles.create_points(t), 2);

        linesContext.moveTo(points[0][0], points[0][1]);

        for (let i = 1; i < points.length; i++) {
            const [x, y] = points[i];
            linesContext.lineTo(x, y);
        }

        linesContext.stroke();
        pointsContext.strokeRect(points[points.length - 1][0], points[points.length - 1][1], 1, 1);
        linesContext.drawImage(pointsCanvas, -halfWidth, -halfHeight);
    };
};

const prepareCanvas = () => {
    const canvas = document.getElementById('canvas')! as HTMLCanvasElement;
    canvas.width = Number(canvas.getBoundingClientRect().width);
    canvas.height = Number(canvas.getBoundingClientRect().height);
    const context = canvas.getContext('2d')!;
    context.translate(canvas.width / 2, canvas.height / 2);
};

window.addEventListener('load', function load() {
    window.removeEventListener('load', load);

    prepareCanvas();
    start();
});

const start = async () => {
    const { RotatingCircles } = await import('../wasm_build/index');
    const rotatingCircles = RotatingCircles.new();

    let animationHandle: number;

    window.addEventListener('beforeunload', () => {
        rotatingCircles.free();
        cancelAnimationFrame(animationHandle);
    });

    const draw = generateDrawFunction(rotatingCircles);
    let t = 0;

    const animate = () => {
        draw(t);
        t = (t + 0.001) % 1;

        animationHandle = requestAnimationFrame(animate);
    };

    animationHandle = requestAnimationFrame(animate);
};
