import type { NextFunction, Request, Response } from "express";

export const errorHandler = (
  err: any,
  _req: Request,
  res: Response,
  _next: NextFunction
): void => {
    const statusCode = err.statusCode || 500;
    res.status(statusCode).json({
      statusCode,
      message: err.message || "Internal Server Error",
      details: err.details ?? undefined,
    });
}
