import { Breadcrumb, Container, Content, Divider } from "rsuite";
import { useNavigate } from "react-router-dom";
import { useTranslation } from "react-i18next";

function Wechat() {
  const navigate = useNavigate();
  const { t, i18n } = useTranslation();
  return (
    <Container>
      <Content>
        <div className="time-machine-body">
          <Breadcrumb
            style={{ marginTop: "5vh", marginBottom: "0", fontSize: "19px" }}
          >
            <Breadcrumb.Item
              onClick={() => {
                navigate("/", { replace: false });
              }}
              href="/"
            >
              {t("home-main-title")}
            </Breadcrumb.Item>
            <Breadcrumb.Item active>
              {t("home-login-wechat-title")}
            </Breadcrumb.Item>
          </Breadcrumb>
          <Divider style={{ marginTop: "1vh" }} />
        </div>
      </Content>
    </Container>
  );
}

export default Wechat;
