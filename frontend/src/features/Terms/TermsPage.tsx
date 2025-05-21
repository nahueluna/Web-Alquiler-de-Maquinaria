import { Box, Divider, Typography } from "@mui/joy";

const TermsPage = () => {
  return (
    <Box
      sx={{
        minWidth: "50%",
        height: "70vh",
        overflow: "auto",
        mx: "auto",
        p: 2,
        border: "1px solid #ccc",
        borderRadius: "8px",
      }}
    >
      <Box sx={{ maxWidth: 800, mx: "auto", p: 4 }}>
        <Typography level="h4" gutterBottom>
          Términos y Condiciones de Uso
        </Typography>
        <Typography level="body-sm" gutterBottom>
          Última actualización: 20 de mayo de 2025
        </Typography>

        <Typography level="body-xs" gutterBottom>
          Bienvenido/a a nuestra plataforma de alquiler de maquinaria. Al
          acceder y utilizar este sitio web, usted acepta cumplir y regirse por
          los siguientes Términos y Condiciones. Si no está de acuerdo con
          alguno de los términos, le solicitamos que no utilice nuestros
          servicios.
        </Typography>

        <Divider sx={{ my: 2 }} />

        <Section title="1. Requisitos de Edad">
          El uso de esta plataforma está restringido exclusivamente a personas
          que tengan 18 años o más. Al registrarse o realizar una reserva, usted
          declara y garantiza que cumple con este requisito de edad. Nos
          reservamos el derecho de solicitar documentación que acredite su
          mayoría de edad en cualquier momento.
        </Section>

        <Section title="2. Objeto del Servicio">
          Nuestra empresa brinda servicios de reserva y alquiler de maquinaria a
          través de esta plataforma. Todas las reservas están sujetas a
          disponibilidad, verificación de identidad del cliente y aprobación por
          parte del personal autorizado.
        </Section>

        <Section title="3. Registro de Usuario">
          Para utilizar los servicios, el usuario debe crear una cuenta
          proporcionando información personal verídica y actualizada. El usuario
          es responsable de mantener la confidencialidad de su cuenta y
          contraseña, así como de todas las actividades que se realicen desde su
          cuenta.
        </Section>

        <Section title="4. Proceso de Reserva">
          Las reservas se pueden realizar a través del sitio web, seleccionando
          la maquinaria deseada, el período de alquiler y completando el proceso
          de pago inicial correspondiente. Nos reservamos el derecho de rechazar
          o cancelar reservas, incluso luego de haber sido confirmadas, por
          motivos de disponibilidad, seguridad, uso indebido del servicio o
          incumplimiento de los Términos y Condiciones.
        </Section>

        <Section title="5. Cancelación por Parte del Empleado">
          El personal de la empresa está autorizado a cancelar un alquiler en
          cualquier momento, ya sea antes o durante el período de alquiler, si
          se detectan irregularidades, mal uso de la maquinaria, condiciones de
          seguridad comprometidas o cualquier situación que así lo justifique.
        </Section>

        <Section title="6. Precios y Modificaciones">
          El precio final del alquiler será confirmado al momento de validar la
          reserva. El valor del alquiler puede estar sujeto a modificaciones si
          se presentan las siguientes situaciones:
          <ul>
            <li>
              Devolución tardía: Se aplicará un interés del 10% diario sobre el
              monto total del alquiler.
            </li>
            <li>
              Daños o uso indebido: Se evaluará un recargo adicional según el
              costo de reparación o reposición.
            </li>
            <li>
              Cambios solicitados por el cliente: El precio podrá ajustarse en
              consecuencia.
            </li>
          </ul>
        </Section>

        <Section title="7. Políticas de Devolución">
          La empresa contempla devoluciones totales, parciales o la no
          devolución según el caso. Estas serán evaluadas por el equipo
          responsable y su decisión será definitiva.
        </Section>

        <Section title="8. Obligaciones del Usuario">
          El usuario se compromete a utilizar la maquinaria para los fines
          autorizados, devolverla en tiempo y forma, no transferir su uso a
          terceros y responder por cualquier daño o pérdida durante el alquiler.
        </Section>

        <Section title="9. Limitación de Responsabilidad">
          La empresa no será responsable por daños directos o indirectos
          derivados del uso de la maquinaria. Su responsabilidad se limita a la
          prestación del servicio de acuerdo con estos términos.
        </Section>

        <Section title="10. Modificaciones">
          Nos reservamos el derecho de modificar estos Términos y Condiciones en
          cualquier momento sin previo aviso. Las modificaciones entrarán en
          vigencia al ser publicadas en este sitio.
        </Section>

        <Section title="11. Legislación Aplicable y Jurisdicción">
          Estos términos se rigen por las leyes de la República Argentina. Para
          cualquier conflicto, las partes se someten a los tribunales ordinarios
          de la Ciudad Autónoma de Buenos Aires.
        </Section>

        <Divider sx={{ my: 2 }} />

        <Typography level="body-xs">
          Al utilizar este sitio y realizar una reserva, usted declara haber
          leído, entendido y aceptado estos Términos y Condiciones en su
          totalidad.
        </Typography>
      </Box>
    </Box>
  );
};

const Section = ({ title, children }) => (
  <Box sx={{ mb: 3 }}>
    <Typography level="h4" gutterBottom>
      {title}
    </Typography>
    <Typography level="body-xs">{children}</Typography>
  </Box>
);

export default TermsPage;
