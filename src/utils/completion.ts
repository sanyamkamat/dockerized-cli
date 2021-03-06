export default class Completion {
    private code: number;

    public constructor(code: number) {
        this.code = code;
    }

    public getCode() {
        return this.code;
    }

    public static of(code: number): Completion {
        return new Completion(code);
    }
}
