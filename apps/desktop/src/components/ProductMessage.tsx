type ProductMessageVariant = "status" | "error";

type ProductMessageProps = {
  variant: ProductMessageVariant;
  message: string;
};

export default function ProductMessage({ variant, message }: ProductMessageProps) {
  if (!message) {
    return null;
  }
  const className = variant === "error" ? "status-error" : "status-ok";
  const role = variant === "error" ? "alert" : "status";
  return (
    <p className={`product-message ${className}`} role={role}>
      {message}
    </p>
  );
}
