import { useEffect } from 'react';
import { checkout } from '@...';

// Create a Checkout SDK instance
const checkoutSDK = new checkout.Checkout();

export function App() {
  // Initialise the Commerce Widget
  useEffect(() => {
    (async () => {
      // Create a factory
      const factory = await checkoutSDK.widgets({
        config: { theme: checkout.WidgetTheme.DARK, language: 'en' },
      });

      // Create a widget
      const widget = factory.create(checkout.WidgetType.i_COMMERCE);

      // Mount a bridge flow, optionally pass any BridgeWidgetParams
      widget.mount('mount-point', {
        flow: checkout.CommerceFlowType.BRIDGE,
      });
    })();
  }, []);

  return <div id="mount-point" />;

     import { useEffect, useState } from 'react';
import { checkout } from '@...';

const checkoutSDK = new checkout.Checkout();

export function App() {
  const [paymentStatus, setPaymentStatus] = useState(null);

  useEffect(() => {
    (async () => {
      const factory = await checkoutSDK.widgets({
        config: { theme: checkout.WidgetTheme.DARK, language: 'en' },
      });

      const widget = factory.create(checkout.WidgetType.i_COMMERCE);

      widget.mount('payment-gateway', {
        flow: checkout.CommerceFlowType.BRIDGE,
        onPaymentResult: (result) => {
          setPaymentStatus(result);
        },
      });
    })();
  }, []);

  return (
    <div>
      <div id="payment-gateway"
}
