import { v2 as cloudinary } from "cloudinary";
import { CloudinaryStorage } from "multer-storage-cloudinary";
import multer from "multer";
import e from "express";
import { AppError } from "../errors/app-error-handler";

// Setup cloudinary configuration using environment variables
// You can get these credentials from your Cloudinary dashboard

cloudinary.config({
  cloud_name: process.env.CLOUDINARY_CLOUD_NAME,
  api_key: process.env.CLOUDINARY_API_KEY,
  api_secret: process.env.CLOUDINARY_API_SECRET,
});

const storage = new CloudinaryStorage({
  cloudinary: cloudinary,
  params: async (_req, _file) => ({
    folder: "your-folder-name", // Change this to the desired folder in Cloudinary
    allowed_formats: ["jpg", "png", "jpeg"],
  }),
});

// File filter function (allows only specific file types)
const fileFilter = (
  req: e.Request,
  file: Express.Multer.File,
  cb: multer.FileFilterCallback,
) => {
  const allowedTypes = ["image/png", "image/jpg", "application/pdf", "video/mp4"];
  if (!allowedTypes.includes(file.mimetype)) {
    return cb(
      new AppError("Invalid input", 400, {
        image: `${file.mimetype} not allowed`,
      }),
    );
  }
  if (file.size > 10 * 1024 * 1024) {
    return cb(
      new AppError("File too large", 400, {
        avatar: "File size should not exceed 10MB",
      }),
    );
  }
  cb(null, true);
};

export const upload = multer({
  storage,
  limits: { fileSize: 10 * 1024 * 1024 },
  fileFilter,
});