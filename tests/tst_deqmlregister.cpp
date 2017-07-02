/*
 * Copyright (C) 2016 Lucien XU <sfietkonstantin@free.fr>
 *
 * You may use this file under the terms of the BSD license as follows:
 *
 * "Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *   * Redistributions of source code must retain the above copyright
 *     notice, this list of conditions and the following disclaimer.
 *   * Redistributions in binary form must reproduce the above copyright
 *     notice, this list of conditions and the following disclaimer in
 *     the documentation and/or other materials provided with the
 *     distribution.
 *   * The names of its contributors may not be used to endorse or promote
 *     products derived from this software without specific prior written
 *     permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE."
 */

#include <DOtherSide/DosQMetaObject.h>
#include <DOtherSideExtra/deqml.h>
#include <DOtherSideExtra/deqmlregister.h>
#include <DOtherSideExtra/deqobjectwrapper.h>
#include <QQmlEngine>
#include <QQuickView>
#include <QtTest/QTest>

class TestQObject : public QObject
{
    Q_OBJECT
public:
    struct Properties
    {
        bool constructorCalled{false};
        bool destructorCalled{false};
    };
    explicit TestQObject(QObject *parent = nullptr)
        : QObject{parent}
    {
    }
    static Properties &properties()
    {
        static Properties s_properties{};
        return s_properties;
    }
    static void createObject(int id, void *wrapper, void **bindedQObject, void **dosQObject)
    {
        Q_UNUSED(id);
        Q_UNUSED(wrapper);
        *bindedQObject = new int{42};

        auto metaObject = std::make_shared<DOS::DosQObjectMetaObject>();
        *dosQObject = new DEQObject(metaObject, nullptr);

        properties().constructorCalled = true;
    }
    static void deleteObject(int id, void *bindedQObject)
    {
        Q_UNUSED(id);
        delete static_cast<int *>(bindedQObject);

        properties().destructorCalled = true;
    }
};

class TestDeQmlRegister : public QObject
{
public:
    Q_OBJECT
private slots:
    void registerType()
    {
        auto metaObject = std::make_shared<DOS::DosQObjectMetaObject>();
        auto createObject = TestQObject::createObject;
        auto deleteObject = TestQObject::deleteObject;
        DOS::QmlRegisterType testObject;
        {
            testObject.major = 1;
            testObject.minor = 0;
            testObject.uri = "testing";
            testObject.qml = "TestQObject";
            testObject.staticMetaObject = metaObject;
            testObject.createDObject = createObject;
            testObject.deleteDObject = deleteObject;
        }
        deQmlRegisterQObject(std::move(testObject));
        DOS::QmlRegisterType otherObject;
        {
            otherObject.major = 1;
            otherObject.minor = 0;
            otherObject.uri = "testing";
            otherObject.qml = "OtherObject";
            otherObject.staticMetaObject = metaObject;
            otherObject.createDObject = createObject;
            otherObject.deleteDObject = deleteObject;
        }
        deQmlRegisterQObject(std::move(otherObject));
    }
    void checkRegistration()
    {
        bool testQObjectRegistered{DEQObjectWrapper<0, RegisterType>::isRegistered()};
        QCOMPARE(testQObjectRegistered, true);
        bool otherObjectRegistered{DEQObjectWrapper<1, RegisterType>::isRegistered()};
        QCOMPARE(otherObjectRegistered, true);

        bool invalidRegistered{DEQObjectWrapper<2, RegisterType>::isRegistered()};
        QCOMPARE(invalidRegistered, false);
        bool invalidUncreatableRegistered{DEQObjectWrapper<0, RegisterUncreatableType>::isRegistered()};
        QCOMPARE(invalidUncreatableRegistered, false);
        bool invalidSingletonRegistered{DEQObjectWrapper<0, RegisterSingletonType>::isRegistered()};
        QCOMPARE(invalidSingletonRegistered, false);
    }
    void testCreation()
    {
        bool done{false};

        QQuickView view{};
        QObject::connect(view.engine(), &QQmlEngine::quit, [&done]() { done = true; });
        view.setSource(QUrl{"qrc:/qml/tst_deqmlregister.qml"});
        view.show();

        while (!done) {
            QTest::qWait(150);
        }

        QCOMPARE(TestQObject::properties().constructorCalled, true);
        QCOMPARE(TestQObject::properties().destructorCalled, true);
    }
};

QTEST_MAIN(TestDeQmlRegister)

#include "tst_deqmlregister.moc"
